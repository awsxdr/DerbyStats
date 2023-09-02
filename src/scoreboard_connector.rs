use std::sync::{Mutex, Arc};
use std::{thread, net::TcpStream};

use bus::{Bus, BusReader};

use serde::{Serialize, Deserialize};
use serde_json::{json, Value, Map};

use websocket::OwnedMessage;
use websocket::{
    ClientBuilder,
    sync::Writer,
    Message
};

pub type ScoreboardState = Map<String, Value>;

struct ScoreboardStateStore {
    pub state: ScoreboardState,
}

#[derive(Serialize, Deserialize)]
pub struct ScoreboardStateUpdate {
    pub state: ScoreboardState,
}

pub struct ScoreboardConnection {
    socket_writer: Writer<TcpStream>,
    bus: Arc<Mutex<Bus<ScoreboardState>>>,
}

impl ScoreboardConnection {
    pub fn new(url: &str) -> Result<ScoreboardConnection, String> {
        
        println!("Opening websocket connection to {}", url);
        let (mut receiver, sender) = 
            ClientBuilder::new(&url).unwrap()
            .connect_insecure().unwrap()
            .split().unwrap();

        let connection = ScoreboardConnection {
            socket_writer: sender,
            bus: Arc::new(Mutex::new(Bus::new(100))),
        };

        let thread_bus = connection.bus.clone();
        thread::spawn(move || {
            let mut state = ScoreboardStateStore::new();

            for message in receiver.incoming_messages() {
                match message {
                    Ok(m) => {
                        state.handle_message(m);
                        thread_bus.lock().unwrap().broadcast(state.state.clone());
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

        self.socket_writer.send_message(&Message::text(message_json.to_string())).unwrap();
    }

    pub fn get_receiver(&mut self) -> BusReader<ScoreboardState> {
        self.bus.lock().unwrap().add_rx()
    }
}

impl ScoreboardStateStore {

    pub fn new() -> ScoreboardStateStore {
        ScoreboardStateStore {
            state: Map::new()
        }
    }

    pub fn handle_message(&mut self, message: OwnedMessage) {
        let message_text = match message {
            OwnedMessage::Text(data) => data,
            _ => {
                println!("Unexpected message type received: {:?}", message);
                return;
            }
        };

        let update: ScoreboardStateUpdate = serde_json::from_str(message_text.as_str()).unwrap();

        for (key, value) in update.state {
            self.state.insert(key, value);
        }
    }
}