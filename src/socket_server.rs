use std::{thread, net::TcpStream, collections::HashMap, sync::{Mutex, Arc}};

use serde::{Serialize, Deserialize};

use serde_json::{Value, json};
use websocket::{sync::{Server, Writer}, OwnedMessage, Message};

pub struct SocketServer {
    subscribers: HashMap<String, Vec<Arc<Mutex<Writer<TcpStream>>>>>,
    update_providers: HashMap<String, Arc<Mutex<dyn UpdateProvider + Send>>>,
}

pub trait UpdateProvider {
    fn get_state(&self) -> Value;
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
}

impl SocketServer {
    pub fn new() -> Arc<Mutex<SocketServer>> {
        let socket_server = Arc::new(Mutex::new(SocketServer {
            subscribers: HashMap::new(),
            update_providers: HashMap::new(),
        }));

        let socket_server_thread_instance = socket_server.clone();

        thread::spawn(move || {

            let server = Server::bind("0.0.0.0:8003").unwrap();

            for request in server.filter_map(Result::ok) {
                let socket_server_loop_instance = socket_server_thread_instance.clone();
                thread::spawn(move || {
                    let client = request.accept().unwrap();
                    let (mut reader, writer) = client.split().unwrap();

                    let writer_arc = Arc::new(Mutex::new(writer));

                    for message in reader.incoming_messages() {
                        match message {
                            Ok(m) => {
                                socket_server_loop_instance.lock().unwrap().handle_message(m, writer_arc.clone());
                            }
                            _ => {
                                return;
                            }
                        }
                    }
                });
            }
        });

        socket_server
    }

    pub fn send_update(&mut self, data_type: &String, update: Value) {
        self.subscribers.get(data_type).map(|handlers| {
            for handler in handlers {
                handler.lock().unwrap().send_message(&Message::text(json!({
                    "dataType": data_type,
                    "body": update
                }).to_string())).unwrap();
            }
        });
    }

    pub fn set_update_provider(&mut self, data_type: &String, provider: Arc<Mutex<dyn UpdateProvider + Send>>) {
        self.update_providers.insert(data_type.clone(), provider);
    }

    fn handle_message(&mut self, message: OwnedMessage, writer: Arc<Mutex<Writer<TcpStream>>>) {
        let message_text = match message {
            OwnedMessage::Text(data) => data,
            _ => {
                println!("Unexpected message type received: {:?}", message);
                return;
            }
        };

        let generic_message: GenericMessage = serde_json::from_str(message_text.as_str()).unwrap();

        match generic_message.message_type.as_str() {
            "Subscribe" => {
                let subscribe_message: SubscribeMessage = match serde_json::from_str(message_text.as_str()) {
                    Ok(v) => { v }
                    _ => { return; }
                };
                self.handle_subscribe(subscribe_message, writer)
            }
            _ => { }
        }
    }

    fn handle_subscribe(&mut self, subscribe_message: SubscribeMessage, writer: Arc<Mutex<Writer<TcpStream>>>) {
        if !self.subscribers.contains_key(&subscribe_message.data_type) {
            self.subscribers.insert(subscribe_message.data_type.clone(), Vec::new());
        }

        self.subscribers.get_mut(&subscribe_message.data_type).unwrap().push(writer.clone());

        if !self.update_providers.contains_key(&subscribe_message.data_type) {
            return;
        }

        let state = self.update_providers[&subscribe_message.data_type].lock().unwrap().get_state();

        writer.lock().unwrap().send_message(&Message::text(json!({
            "dataType": subscribe_message.data_type,
            "body": state
        }).to_string())).unwrap();
    }
}