use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use log::{trace, debug, error};
use regex::Regex;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use crate::{socket_server::{UpdateProvider, Update, SocketServer}, scoreboard_connector::{ScoreboardConnection, ScoreboardState}};

type PenaltyCountMap = HashMap<String, u32>;

#[derive(Serialize, Deserialize, Clone)]
struct PenaltyDetails {
    period_number: u8,
    team: u8,
    jam_number: u32,
    skater_id: String,
    penalty_code: String,
}

impl PenaltyDetails {
    fn new(penalty: &PenaltyMatches) -> PenaltyDetails {
        PenaltyDetails { period_number: 0, team: penalty.team, jam_number: 0, skater_id: "".to_string(), penalty_code: "".to_string() }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct GamePenaltyDetails {
    #[serde(rename = "codes")]
    codes: HashMap<String, String>,

    #[serde(rename = "penalties")]
    penalties: HashMap<(String, u32), PenaltyDetails>,
}

impl GamePenaltyDetails {
    fn new() -> GamePenaltyDetails {
        GamePenaltyDetails {
            codes: HashMap::new(),
            penalties: HashMap::new(),
        }
    }
}

#[derive(Clone)]
struct PenaltyCodeMatches {
    game_id: String,
    code: String,
    name: String,
}

#[derive(Clone)]
struct PenaltyMatches {
    penalty_id: u32,
    game_id: String,
    team: u8,
    skater_id: String,
    property_name: String,
    value: String,
}

#[derive(Clone)]
enum Match {
    PenaltyCode(PenaltyCodeMatches),
    Penalty(PenaltyMatches),
}

#[derive(Serialize, Deserialize)]
struct PenaltyStates {
    #[serde(rename = "penaltyCountsByTypeByTeam")]
    pub penalty_counts_by_type_by_team: HashMap<u8, PenaltyCountMap>,

    #[serde(rename = "penaltyCountsByJamByTeam")]
    pub penalty_counts_by_jam_by_team: HashMap<i32, (u32, u32)>,
}

pub struct PenaltiesByType {
    game_states: HashMap<String, PenaltyStates>,
    penalty_code_regex: Regex,
    penalty_regex: Regex,
}

impl PenaltiesByType {
    pub async fn new(scoreboard: &mut ScoreboardConnection, socket_server: &mut SocketServer) {
        let penalties_by_type = Arc::new(Mutex::new(PenaltiesByType { 
            game_states: HashMap::new(),
            penalty_code_regex: Regex::new(r#"^ScoreBoard\.Game\(([^\)]+)\)\.PenaltyCode\((.)\)$"#).unwrap(),
            penalty_regex: Regex::new(r#"^ScoreBoard\.Game\(([^\)]+)\)\.Team\((\d+)\)\.Skater\(([^\)]+)\)\.Penalty\(([^\)]+)\)\.([^\.]+)$"#).unwrap(),
        }));

        let mut receiver = scoreboard.get_receiver();

        let update_sender = socket_server.get_update_sender();
        socket_server.register_update_provider(&"PenaltiesByType".to_string(), penalties_by_type.clone()).await;

        tokio::task::spawn(async move {
            while let Ok(state_update) = receiver.recv().await {
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

        scoreboard.register_topic("ScoreBoard.Game(*).PenaltyCode(*)");
        scoreboard.register_topic("ScoreBoard.Game(*).Team(*).Skater(*).Penalty(*).Code");
        scoreboard.register_topic("ScoreBoard.Game(*).Team(*).Skater(*).Penalty(*).JamNumber");
        scoreboard.register_topic("ScoreBoard.Game(*).Team(*).Skater(*).Penalty(*).PeriodNumber");
    }

    fn process_state_update(&mut self, update: ScoreboardState) -> Vec<String> {
        debug!("Processing stats update for penalties by code");

        let game_penalty_details = update.iter()
            .filter_map(|u| self.get_relevant_states(u))
            .fold(HashMap::new(), |mut map, match_info| {
                let game_id = match match_info.clone() {
                    Match::PenaltyCode(penalty_code) => penalty_code.game_id,
                    Match::Penalty(penalty) => penalty.game_id,
                };

                if !map.contains_key(&game_id) {
                    map.insert(game_id.clone(), GamePenaltyDetails::new());
                }

                let game_penalties = map.get_mut(&game_id).unwrap();

                match match_info {
                    Match::PenaltyCode(penalty_code) => {
                        game_penalties.codes.insert(penalty_code.code.clone(), penalty_code.name.clone());
                    },
                    Match::Penalty(penalty) => {
                        let key = (penalty.skater_id.clone(), penalty.penalty_id.clone());
                        if !game_penalties.penalties.contains_key(&key) {
                            game_penalties.penalties.insert(key.clone(), PenaltyDetails::new(&penalty));
                        }
                        let penalty_details = game_penalties.penalties.get_mut(&key).unwrap();

                        match penalty.property_name.as_str() {
                            "Code" => { penalty_details.penalty_code = penalty.value.clone() },
                            "PeriodNumber" => { penalty_details.period_number = penalty.value.parse::<u8>().unwrap() },
                            "JamNumber" => { penalty_details.jam_number = penalty.value.parse::<u32>().unwrap() },
                            _ => { }
                        }
                    }
                }

                map
            });
        
        for (game_id, game_penalties) in game_penalty_details.clone() {
            let make_penalty_code_map = || HashMap::<String, u32>::from_iter(game_penalties.codes.iter().map(|(k, _)| (k.clone(), 0)));

            let penalty_counts_by_type_by_team: HashMap<u8, HashMap<String, u32>> = game_penalties.penalties.iter()
                .fold(HashMap::from([(1, make_penalty_code_map()), (2, make_penalty_code_map())]), |mut map, (_, penalty)| {
                    let team_map = map.get_mut(&penalty.team).unwrap();

                    trace!("Penalty {} for team {} in P {}, J {}", penalty.penalty_code, penalty.team, penalty.period_number, penalty.jam_number);

                    match team_map.get_mut(&penalty.penalty_code) {
                        Some(count) => {
                            *count = *count + 1;
                        },
                        None => {
                            error!("Unexpected penalty code encountered for team {}: {}", penalty.team, penalty.penalty_code);
                        }
                    }

                    map
                });

            self.game_states.insert(game_id.clone(), PenaltyStates {
                penalty_counts_by_type_by_team,
                penalty_counts_by_jam_by_team: HashMap::new(),
            });
        }

        game_penalty_details.keys().map(|k| k.clone()).collect()
    }

    fn get_relevant_states(&self, (key, value): (&String, &Value)) -> Option<Match> {
        if self.penalty_code_regex.is_match(key) {
            trace!("Received penalty code update");
            self.penalty_code_regex.captures(key).map(|c| {
                let (_, [game_id, code]) = c.extract();

                Match::PenaltyCode(PenaltyCodeMatches {
                    game_id: game_id.to_string(),
                    code: code.to_string(),
                    name: value.as_str().unwrap().split(",").next().unwrap().to_string(),
                })
            })
        } else if self.penalty_regex.is_match(key) {
            trace!("Received penalty update");

            self.penalty_regex.captures(key).map(|c| {
                let (_, [game_id, team, skater, penalty_id, property]) = c.extract();

                Match::Penalty(PenaltyMatches {
                    penalty_id: penalty_id.parse::<u32>().unwrap(),
                    game_id: game_id.to_string(),
                    team: team.parse::<u8>().unwrap(),
                    skater_id: skater.to_string(),
                    property_name: property.to_string(),
                    value: value.as_str().map(|s| s.to_string()).or_else(|| value.as_u64().map(|s| s.to_string())).unwrap(),
                })
            })
        } else {
            None
        }
    }
}

impl UpdateProvider for PenaltiesByType {
    fn get_state(&self, game_id: &String) -> serde_json::Value {
        json!(self.game_states.get(game_id).unwrap().clone())
    }
}
