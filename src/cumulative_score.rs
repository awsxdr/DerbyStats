use std::{thread, collections::HashMap, sync::{Mutex, Arc}};

use regex::Regex;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use crate::{socket_server::{SocketServer, UpdateProvider}, scoreboard_connector::{ScoreboardConnection, ScoreboardState}};

pub struct CumulativeScore {
    state: Value,
}

#[derive(Serialize, Deserialize, Clone)]
struct JamScore {
    #[serde(rename = "jamNumber")]
    jam_number: i32,
    #[serde(rename = "team1Score")]
    team_1_score: i64,
    #[serde(rename = "team2Score")]
    team_2_score: i64,
}

impl CumulativeScore {
    pub fn new(mut scoreboard: ScoreboardConnection, socket_server: Arc<Mutex<SocketServer>>) {
        let cumulative_score = Arc::new(Mutex::new(CumulativeScore { 
            state: json!({
                "jamScores": []
            }),
        }));

        socket_server.lock().unwrap().set_update_provider(&"CumulativeScore".to_string(), cumulative_score.clone());

        scoreboard.register_topic("ScoreBoard.CurrentGame.Period(*).Jam(*).TeamJam(*).TotalScore");

        let mut receiver = scoreboard.get_receiver();
        thread::spawn(move || {
            for state_update in receiver.iter() {
                cumulative_score.lock().unwrap().process_state_update(state_update);
                socket_server.lock().unwrap().send_update(
                    &"CumulativeScore".to_string(), 
                    cumulative_score.lock().unwrap().state.clone());
            }
        });
    }

    fn process_state_update(&mut self, update: ScoreboardState) {
        let total_score_regex = Regex::new(r#"ScoreBoard\.CurrentGame\.Period\((\d+)\)\.Jam\((\d+)\)\.TeamJam\((\d+)\)\.TotalScore"#).unwrap();
        
        let mut scores: Vec<JamScore> = update.iter()
            .filter_map(|(k, v)| {
                total_score_regex.captures(k).map(
                    |c| {
                        let (_, [period, jam, team]) = c.extract();
                        
                        (period, jam, team, v)
                })
            })
            .map(|(period, jam, team, value)| {(
                period.parse::<i32>().unwrap(),
                jam.parse::<i32>().unwrap(),
                team.parse::<i32>().unwrap(),
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
            })
            .values()
            .cloned()
            .collect();

        scores.sort_by(|a, b| a.jam_number.partial_cmp(&b.jam_number).unwrap());

        self.state = json!({
            "jamScores": scores                
        });
    }
}

impl UpdateProvider for CumulativeScore {
    fn get_state(&self) -> serde_json::Value {
        self.state.clone()
    }
}