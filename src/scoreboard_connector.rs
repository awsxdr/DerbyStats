use std::collections::HashMap;
use std::{thread, net::TcpStream};

use log::{debug, info, warn, trace};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use tokio::sync::broadcast::{self, Sender, Receiver};
use websocket::OwnedMessage;
use websocket::{
    ClientBuilder,
    sync::Writer,
    Message
};

pub type ScoreboardState = HashMap<String, Value>;

struct ScoreboardStateStore {
    pub state: ScoreboardState,
}

#[derive(Serialize, Deserialize)]
pub struct ScoreboardStateUpdate {
    pub state: ScoreboardState,
}

pub struct ScoreboardConnection {
    socket_writer: Writer<TcpStream>,
    state_sender: Sender<ScoreboardState>,
}

impl ScoreboardConnection {
    pub fn new(url: String) -> Result<ScoreboardConnection, String> {
        
        info!("Opening scoreboard websocket connection to {}", url);
        let (mut receiver, sender) = 
            ClientBuilder::new(&url).unwrap()
            .connect_insecure().unwrap()
            .split().unwrap();

        let (state_sender, _) = broadcast::channel(100);
        let thread_sender = state_sender.clone();

        let connection = ScoreboardConnection {
            socket_writer: sender,
            state_sender,
        };

        thread::spawn(move || {
            let mut state = ScoreboardStateStore::new();

            for message in receiver.incoming_messages() {
                match message {
                    Ok(m) => {
                        state.handle_message(m);
                        thread_sender.send(state.state.clone()).unwrap();
                    },
                    _ => {}
                };
            }
        });

        Ok(connection)
    }

    pub fn register_topic(&mut self, topic_name: &str) {
        let message_json = json!({
            "action": "Register",
            "paths": [
                topic_name
            ]
        });

        debug!("Registering topic {}", topic_name);

        self.socket_writer.send_message(&Message::text(message_json.to_string())).unwrap();
    }

    pub fn get_receiver(&mut self) -> Receiver<ScoreboardState> {
        self.state_sender.subscribe()
    }
}

impl ScoreboardStateStore {

    pub fn new() -> ScoreboardStateStore {
        ScoreboardStateStore {
            state: HashMap::new()
        }
    }

    pub fn handle_message(&mut self, message: OwnedMessage) {
        let message_text = match message {
            OwnedMessage::Text(data) => data,
            _ => {
                warn!("Unexpected message type received: {:?}", message);
                return;
            }
        };

        trace!("Update received: {}", message_text);

        let update: ScoreboardStateUpdate = serde_json::from_str(message_text.as_str()).unwrap();

        for (key, value) in update.state {
            trace!("State update received for {}", key);
            
            self.state.insert(key, value);
        }
    }
}