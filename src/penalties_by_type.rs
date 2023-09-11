use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use log::{debug, error};
use regex::Regex;
use serde::{Serialize, Deserialize};
use serde_json::json;

use crate::{socket_server::{UpdateProvider, Update, SocketServer}, scoreboard_connector::{ScoreboardConnection, ScoreboardState}};

type PenaltyCountMap = HashMap<String, u32>;

#[derive(Serialize, Deserialize)]
struct PenaltyStates {
    #[serde(rename = "penaltyCountsByTypeByTeam")]
    pub penalty_counts_by_type_by_team: HashMap<i32, PenaltyCountMap>,

    #[serde(rename = "penaltyCountsByJamByTeam")]
    pub penalty_counts_by_jam_by_team: HashMap<i32, (u32, u32)>,
}

pub struct PenaltiesByType {
    game_states: HashMap<String, PenaltyStates>,
}

impl PenaltiesByType {
    pub async fn new(scoreboard: &mut ScoreboardConnection, socket_server: &mut SocketServer) {
        let penalties_by_type = Arc::new(Mutex::new(PenaltiesByType { 
            game_states: HashMap::new(),
        }));

        let mut receiver = scoreboard.get_receiver();

        let update_sender = socket_server.get_update_sender();
        socket_server.register_update_provider(&"PenaltiesByType".to_string(), penalties_by_type.clone()).await;

        tokio::task::spawn(async move {
            for state_update in receiver.iter() {
                let mut locked_penalties = penalties_by_type.lock().await;

                let update_game_ids = locked_penalties.process_state_update(state_update);

                debug!("{} games updated", update_game_ids.len());
                for update_game_id in update_game_ids {
                    let update = if let Some(s) = locked_penalties.game_states.get(&update_game_id) {
                        s.clone()
                    } else {
                        continue;
                    };

                    debug!("Sending PenaltiesByType update for game {}", update_game_id.clone());
                    if let Err(e) = update_sender.send(Update { game_id: update_game_id, data_type: "PenaltiesByType".to_string(), update: json!(update.clone())}) {
                        error!("Error sending update on mpsc: {:?}", e);
                    }
                }
            }
        });

        scoreboard.register_topic("ScoreBoard.Game(*).Team(*).Skater(*).Penalty(*).Code");
    }

    fn process_state_update(&mut self, update: ScoreboardState) -> Vec<String> {
        let penalty_code_regex = Regex::new(r#"ScoreBoard\.Game\(([^\)]+)\)\.Team\((\d+)\)\.Skater\(([^\)]+)\)\.Penalty\((\d+)\)\.Code"#).unwrap();

        debug!("Processing stats update for penalties by code");

        let game_penalty_codes = update.iter()
            .filter_map(|(k, v)|
                penalty_code_regex.captures(k).map(
                    |c| {
                        let (_, [game, team, skater, _penalty]) = c.extract();

                        (game, team, skater, v)
                    }
                ))
            .map(|(game, team, skater, code)| (
                game.to_string(),
                team.parse::<i32>().unwrap(),
                skater.to_string(),
                code.as_str().unwrap().to_string().to_uppercase(),
            ))
            .fold(HashMap::new(), |mut map, (game_id, team, skater, code)| {
                if !map.contains_key(&game_id) {
                    map.insert(game_id.clone(), Vec::new());
                }

                let game_penalties = map.get_mut(&game_id).unwrap();

                game_penalties.push((team, skater, code));

                map
            });
        
        for game_id in game_penalty_codes.keys() {
            let game_penalties = game_penalty_codes.get(game_id).unwrap();

            let penalty_counts_by_type_by_team = game_penalties.iter()
                .fold(HashMap::from([(1, Self::get_new_penalty_map()), (2, Self::get_new_penalty_map())]), |mut map, (team, _skater, code)| {
                    let team_map = map.get_mut(team).unwrap();

                    match team_map.get_mut(code) {
                        Some(count) => {
                            *count = *count + 1;
                        },
                        None => {
                            error!("Unexpected penalty code encountered for team {}: {}", team, code);
                        }
                    }

                    map
                });

            self.game_states.insert(game_id.clone(), PenaltyStates {
                penalty_counts_by_type_by_team,
                penalty_counts_by_jam_by_team: HashMap::new(),
            });
        }

        game_penalty_codes.keys().map(|k| k.clone()).collect()
    }

    fn get_new_penalty_map() -> PenaltyCountMap {
        HashMap::from([
            ("A".to_string(), 0),
            ("B".to_string(), 0),
            ("C".to_string(), 0),
            ("D".to_string(), 0),
            ("E".to_string(), 0),
            ("F".to_string(), 0),
            ("G".to_string(), 0),
            ("I".to_string(), 0),
            ("L".to_string(), 0),
            ("M".to_string(), 0),
            ("O".to_string(), 0),
            ("P".to_string(), 0),
            ("X".to_string(), 0),
            ("Z".to_string(), 0),
        ])
    }
}

impl UpdateProvider for PenaltiesByType {
    fn get_state(&self, game_id: &String) -> serde_json::Value {
        json!(self.game_states.get(game_id).unwrap().clone())
    }
}
