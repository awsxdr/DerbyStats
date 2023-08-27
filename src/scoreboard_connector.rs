use serde_json::json;
use serde::{Deserialize, Serialize};

use websocket::{
    ClientBuilder,
    stream::sync::NetworkStream,
    client::sync::Client,
    Message,
    OwnedMessage
};

pub struct ScoreboardConnection {
    socket: Client<Box<dyn NetworkStream + Send>>
}

impl ScoreboardConnection {
    pub fn new(url: &str) -> Result<ScoreboardConnection, String> {
        let mut builder = match ClientBuilder::new(&url) {
            Ok(b) => b,
            Err(e) => return Err(e.to_string())
        };

        let socket = match builder.connect(None) {
            Ok(s) => s,
            Err(e) => return Err(e.to_string())
        };

        Ok(ScoreboardConnection {
            socket: socket
        })
    }

    pub fn register_topic(&mut self, topic_name: &str) {
        let message_json = json!({
            "action": "Register",
            "paths": [
                topic_name
            ]
        });

        self.socket.send_message(&Message::text(message_json.to_string())).unwrap();
    }
}

#[derive(Serialize, Deserialize)]
pub struct ScoreboardMessage {
}

impl Iterator for ScoreboardConnection {
    type Item = ScoreboardMessage;

    fn next(&mut self) -> Option<Self::Item> {
        let message_result = self.socket.recv_message();

        let message = match message_result {
            Ok(m) => m,
            Err(e) => {
                println!("Error in message loop {:?}", e);
                return None;
            }
        };
        
        let message_text = match message {
            OwnedMessage::Text(data) => data,
            _ => {
                println!("Unexpected message type received: {:?}", message);
                return None;
            }
        };

        println!("{}", message_text);

        //let json_object: ScoreboardMessage = serde_json::from_str(message_text.as_str()).ok()?;
        //Some(json_object)

        Some(ScoreboardMessage {})
    }
}