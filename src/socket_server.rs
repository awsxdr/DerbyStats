use std::{sync::{Arc, atomic::{AtomicUsize, Ordering}}, collections::HashMap};

use futures_util::{SinkExt, StreamExt};
use log::{error, trace, debug};
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use tokio::sync::{mpsc, RwLock, Mutex};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::Filter;
use warp::ws::{WebSocket, Message};

type Connections = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Update>>>>;
type Subscribers = RwLock<Vec<usize>>;
type SubscribeChannel = (SubscribeSender, SubscribeReceiver);
type SubscribeSender = mpsc::UnboundedSender<(usize, SubscribeMessage)>;
type SubscribeReceiver = mpsc::UnboundedReceiver<(usize, SubscribeMessage)>;
type SubscriptionKey = (String, String);
type Subscriptions = Arc<RwLock<HashMap<SubscriptionKey, Subscribers>>>;
type UpdateChannel = (UpdateSender, UpdateReceiver);
pub type UpdateSender = mpsc::UnboundedSender<Update>;
type UpdateReceiver = mpsc::UnboundedReceiver<Update>;
type UpdateProviders = Arc<RwLock<HashMap<String, Arc<Mutex<dyn UpdateProvider + Send>>>>>;

pub trait UpdateProvider {
    fn get_state(&self, game_id: &String) -> Value;
}

#[derive(Serialize, Deserialize)]
struct GenericMessage {
    #[serde(rename = "messageType")]
    pub message_type: String,
}

#[derive(Serialize, Deserialize)]
struct SubscribeMessage {
    #[serde(rename = "messageType")]
    pub message_type: String,
    #[serde(rename = "dataType")]
    pub data_type: String,
    #[serde(rename = "gameId")]
    pub game_id: String,
}

#[derive(Clone)]
pub struct Update {
    pub game_id: String,
    pub data_type: String,
    pub update: Value
}

static NEXT_CONNECTION_ID: AtomicUsize = AtomicUsize::new(0);

pub struct SocketServer {
    update_sender: UpdateSender,
    update_receiver: UpdateReceiver,
    update_providers: UpdateProviders,
}

impl SocketServer {

    pub fn new() -> SocketServer {
        let (update_sender, update_receiver) = mpsc::unbounded_channel();

        SocketServer { 
            update_sender,
            update_receiver,
            update_providers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get_update_sender(&self) -> UpdateSender {
        debug!("Creating new update sender");
        self.update_sender.clone()
    }

    pub async fn register_update_provider(&mut self, data_type: &String, update_provider: Arc<Mutex<dyn UpdateProvider + Send>>) {
        self.update_providers.write().await.insert(data_type.clone(), update_provider);
    }

    pub async fn listen(mut self, port: u16) {
        let connections = Connections::default();
        let subscriptions = Subscriptions::default();

        let thread_connections = connections.clone();
        let thread_subscriptions = subscriptions.clone();

        tokio::task::spawn(async move {
            debug!("Starting update receiver thread");
            while let Some(update) = self.update_receiver.recv().await {
                let key = (update.game_id.clone(), update.data_type.clone());

                trace!("Update receiver forwarding update of type {}", update.data_type.clone());

                if let Some(subscription) = thread_subscriptions.read().await.get(&key) {
                    for subscriber in subscription.read().await.clone().into_iter() {
                        if let Some(subscriber) = thread_connections.read().await.get(&subscriber) {
                            trace!("Sending update from update receiver thread");
                            if let Err(e) = subscriber.send(update.clone()) {
                                error!("Error sending update across mpsc: {:?}", e);
                            }
                        }
                    }
                }
            }
        });

        let connections = warp::any().map(move || connections.clone());
        let subscriptions = warp::any().map(move || subscriptions.clone());
        let update_providers = warp::any().map(move || self.update_providers.clone());

        let websocket_path = warp::path("ws")
            .and(warp::ws())
            .and(connections)
            .and(subscriptions)
            .and(update_providers)
            .map(|ws: warp::ws::Ws, connections, subscriptions, update_providers| {
                ws.on_upgrade(move |websocket| Self::socket_connected(websocket, connections, subscriptions, update_providers))
            });

        let ui_path = std::env::current_exe().unwrap().parent().unwrap().join("ui");

        debug!("Serving UI from {}", ui_path.to_str().unwrap());
        let default_path = warp::path::end().and(warp::fs::dir(ui_path.clone()));
        let ui_files = warp::fs::dir(ui_path.clone());

        let cors_configuration =
            warp::cors()
                .allow_any_origin()
                .allow_methods(vec!["GET", "OPTIONS"]);

        let routes = websocket_path.or(default_path).or(ui_files).with(cors_configuration);

        warp::serve(routes).run(([0, 0, 0, 0], port)).await;
    }

    async fn socket_connected(websocket: WebSocket, connections: Connections, subscriptions: Subscriptions, update_providers: UpdateProviders) {
        let connection_id = NEXT_CONNECTION_ID.fetch_add(1, Ordering::Relaxed);

        debug!("New client connected and assigned ID: {}", connection_id);

        let (mut client_sender, mut client_receiver) = websocket.split();

        let (subscribe_sender, subscribe_receiver): SubscribeChannel = mpsc::unbounded_channel();
        let mut subscribe_receiver = UnboundedReceiverStream::new(subscribe_receiver);

        let thread_connections = connections.clone();

        tokio::task::spawn(async move {
            while let Some((connection_id, subscribe_request)) = subscribe_receiver.next().await {
                debug!("Processing subscription request on {} for connection {}", subscribe_request.data_type.clone(), connection_id);

                let mut subscriptions = subscriptions.write().await;
                let subscription_key: SubscriptionKey = (subscribe_request.game_id.clone(), subscribe_request.data_type.clone());

                if !subscriptions.contains_key(&subscription_key) {
                    subscriptions.insert(subscription_key.clone(), Subscribers::default());
                }

                let mut subscribers = subscriptions.get(&subscription_key).unwrap().write().await;

                if !subscribers.contains(&connection_id) {
                    subscribers.push(connection_id.clone())
                }

                if let Some(provider) = update_providers.read().await.get(&subscribe_request.data_type) {
                    if let Some(connection) = thread_connections.read().await.get(&connection_id) {
                        let state = provider.lock().await.get_state(&subscribe_request.game_id);
                        if let Err(e) = connection.send(Update {
                            game_id: subscribe_request.game_id.clone(),
                            data_type: subscribe_request.data_type.clone(),
                            update: state,
                        }) {
                            error!("Error sending initial state to connection {}: {:?}", connection_id, e);
                        };
                    }
                }
            }
        });

        let (update_sender, update_receiver): UpdateChannel = mpsc::unbounded_channel();
        let mut update_receiver = UnboundedReceiverStream::new(update_receiver);

        tokio::task::spawn(async move {
            while let Some(update) = update_receiver.next().await {
                debug!("Update received for {}. Sending to {}", update.data_type.clone(), connection_id.clone());

                if let Err(e) = client_sender.send(Message::text(Self::get_update_json(&update).to_string())).await {
                    error!("Error sending update to client {}: {:?}", connection_id, e);
                }
            }
        });

        connections.write().await.insert(connection_id, update_sender);

        while let Some(result) = client_receiver.next().await {
            let message = match result {
                Ok(message) => message,
                Err(e) => {
                    error!("Error receiving data from websocket id {}: {:?}", connection_id, e);
                    break;
                }
            };

            Self::handle_socket_message(connection_id, message, &subscribe_sender).await;
        }

        debug!("Connection {} disconnected", connection_id);
    }

    fn get_update_json(update: &Update) -> Value {
        json!({
            "dataType": update.data_type,
            "body": update.update,
        })
    }

    async fn handle_socket_message(connection_id: usize, message: Message, subscribe_sender: &mpsc::UnboundedSender<(usize, SubscribeMessage)>) {
        let message_text = if let Ok(s) = message.to_str() {
            s
        } else {
            return;
        };

        trace!("Received message: {}", message_text);

        let generic_message: GenericMessage = 
            if let Ok(m) = serde_json::from_str(message_text) {
                m
            } else { 
                return; 
            };

        match generic_message.message_type.as_str() {
            "Subscribe" => {
                let subscribe_message: SubscribeMessage = match serde_json::from_str(message_text) {
                    Ok(v) => { v }
                    Err(e) => { 
                        error!("Failed to parse Subscribe message: {:?}", e);
                        return; 
                    }
                };
                if let Err(e) = subscribe_sender.send((connection_id, subscribe_message)) {
                    error!("Error sending subscribe request on mpsc: {:?}", e);
                }
            },
            _ => {
                return;
            }
        }
    }

}