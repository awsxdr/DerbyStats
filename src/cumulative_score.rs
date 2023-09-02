use std::{thread, collections::HashMap, sync::{Mutex, Arc}};

use regex::Regex;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use crate::{socket_server::{SocketServer, UpdateProvider}, scoreboard_connector::{ScoreboardConnection, ScoreboardState}};

pub struct CumulativeScore {
    game_states: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Clone)]
struct JamScore {
    #[serde(rename = "periodNumber")]
    period_number: i32,
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
            game_states: HashMap::new(),
        }));

        socket_server.lock().unwrap().set_update_provider(&"CumulativeScore".to_string(), cumulative_score.clone());

        let mut receiver = scoreboard.get_receiver();
        thread::spawn(move || {
            for state_update in receiver.iter() {
                let update_game_ids = cumulative_score.lock().unwrap().process_state_update(state_update);

                for update_game_id in update_game_ids {
                    socket_server.lock().unwrap().send_update(
                        &update_game_id,
                        &"CumulativeScore".to_string(), 
                        cumulative_score.lock().unwrap().get_state(&update_game_id));
                }
            }
        });

        scoreboard.register_topic("ScoreBoard.Game(*).Period(*).Jam(*).TeamJam(*).TotalScore");
    }

    fn process_state_update(&mut self, update: ScoreboardState) -> Vec<String> {
        let total_score_regex = Regex::new(r#"ScoreBoard\.Game\(([^\)]+)\)\.Period\((\d+)\)\.Jam\((\d+)\)\.TeamJam\((\d+)\)\.TotalScore"#).unwrap();

        println!("Processing stats update for cumulative score");
        
        let scores = update.iter()
            .filter_map(|(k, v)| {
                total_score_regex.captures(k).map(
                    |c| {
                        let (_, [game, period, jam, team]) = c.extract();
                        
                        (game, period, jam, team, v)
                })
            })
            .map(|(game, period, jam, team, value)| {(
                game.to_string(),
                period.parse::<i32>().unwrap(),
                jam.parse::<i32>().unwrap(),
                team.parse::<i32>().unwrap(),
                value.as_i64().unwrap()
            )})
            .fold(HashMap::new(), |mut map, (game_id, period, jam, team, value)| {
                if !map.contains_key(&game_id) {
                    map.insert(game_id.clone(), HashMap::new());
                }

                let game_map = map.get_mut(&game_id).unwrap();

                let jam_key = (period, jam);

                if !game_map.contains_key(&jam_key) {
                    game_map.insert(jam_key, JamScore {
                        period_number: period,
                        jam_number: jam,
                        team_1_score: 0,
                        team_2_score: 0
                    });
                }

                if team == 1 {
                    game_map.get_mut(&jam_key).unwrap().team_1_score = value
                } else {
                    game_map.get_mut(&jam_key).unwrap().team_2_score = value
                }

                map
            });
        
        for game_id in scores.keys() {
            let mut scores_vector: Vec<JamScore> = scores
                .get(game_id).unwrap()
                .values()
                .cloned()
                .collect();

            scores_vector.sort_by(|a, b| a.jam_number.partial_cmp(&b.jam_number).unwrap());

            let game_id_string = game_id.to_string();

            self.game_states.insert(game_id_string.clone(), json!({
                "jamScores": scores_vector                
            }));

            println!("Set cumulative score state for game {}", game_id);
        }

        scores.keys().map(|k| k.clone()).collect()
    }
}

impl UpdateProvider for CumulativeScore {
    fn get_state(&self, game_id: &String) -> serde_json::Value {
        self.game_states.get(game_id).unwrap().clone()
    }
}