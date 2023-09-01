use std::{thread, collections::HashMap, sync::{Mutex, Arc}};

use regex::Regex;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use crate::{socket_server::{SocketServer, UpdateProvider}, scoreboard_connector::{ScoreboardConnection, ScoreboardState}};

struct CumulativeScore {
    state: Value,
}

#[derive(Serialize, Deserialize)]
struct JamScore {
    jam_number: i32,
    team_1_score: i64,
    team_2_score: i64,
}

impl CumulativeScore {
    pub fn new(mut scoreboard: ScoreboardConnection, mut socket_server: SocketServer) {
        let cumulative_score = Arc::new(Mutex::new(CumulativeScore { 
            state: json!({
                "jamScores": []
            }),
        }));

        socket_server.set_update_provider(&"CumulativeScore".to_string(), cumulative_score.clone());

        scoreboard.register_topic("ScoreBoard.CurrentGame.Period(*).Jam(*).TeamJam(*).TotalScore");

        let mut receiver = scoreboard.get_receiver();
        thread::spawn(move || {
            for state_update in receiver.iter() {
                cumulative_score.lock().unwrap().process_state_update(state_update);
            }
        });
    }

    fn process_state_update(&mut self, update: ScoreboardState) {
        let total_score_regex = Regex::new("").unwrap();
        
        let scores = update.iter()
            .filter_map(|(k, v)| {
                total_score_regex.captures(k).map(
                    |c| (
                        c.iter(),
                        v
                    ))
            })
            .map(|(mut matches, value)| {(
                matches.nth(0).unwrap().unwrap().as_str().parse::<i32>().unwrap(),
                matches.nth(1).unwrap().unwrap().as_str().parse::<i32>().unwrap(),
                matches.nth(2).unwrap().unwrap().as_str().parse::<i32>().unwrap(),
                value.as_i64().unwrap()
            )})
            .fold(HashMap::new(), |mut current, item| {
                if !current.contains_key(&item.1) {
                    current.insert(item.1.clone(), JamScore {
                        jam_number: item.1,
                        team_1_score: 0,
                        team_2_score: 0
                    });
                }

                if item.2 == 1 {
                    current.get_mut(&item.1).unwrap().team_1_score = item.3
                } else {
                    current.get_mut(&item.1).unwrap().team_2_score = item.3
                }

                current
            });

        self.state = json!({
            "jamScores":
                scores
        });
    }
}

impl UpdateProvider for CumulativeScore {
    fn get_state(&self) -> serde_json::Value {
        self.state.clone()
    }
}