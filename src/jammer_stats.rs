use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use log::{debug, error};
use regex::Regex;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use crate::{socket_server::{UpdateProvider, Update, SocketServer}, scoreboard_connector::{ScoreboardConnection, ScoreboardState}};

#[derive(Clone)]
struct JammerScoreMatches {
    game: String,
    jam: u32,
    team: u8,
    score: u64,
}

#[derive(Clone)]
struct JammerSkaterMatches {
    game: String,
    jam: u32,
    team: u8,
    skater_id: String,
}

#[derive(Clone)]
struct SkaterNameMatches {
    game: String,
    team: u8,
    skater_id: String,
    name: String
}

#[derive(Clone)]
enum Match { 
    JammerScore(JammerScoreMatches),
    JammerSkater(JammerSkaterMatches),
    SkaterName(SkaterNameMatches),
}

#[derive(Serialize, Deserialize)]
struct JammerInfo {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "team")]
    team: u8,

    #[serde(rename = "jamCount")]
    jam_count: usize,

    #[serde(rename = "totalScore")]
    total_score: u64,

    #[serde(rename = "meanNetPerJam")]
    mean_net_per_jam: f32,

    #[serde(rename = "leadCount")]
    lead_count: u32,

    #[serde(rename = "meanTimeToInitial")]
    mean_time_to_initial: f32,
}

#[derive(Serialize, Deserialize)]
struct JammerStatsStates {
    #[serde(rename = "jammers")]
    pub jammers: Vec<JammerInfo>,
}

pub struct JammerStats {
    game_states: HashMap<String, JammerStatsStates>,
    jam_score_regex: Regex,
    jam_skater_regex: Regex,
    skater_name_regex: Regex,
}


impl JammerStats {

    pub async fn new(scoreboard: &mut ScoreboardConnection, socket_server: &mut SocketServer) {
        let jammer_stats = Arc::new(Mutex::new(JammerStats { 
            game_states: HashMap::new(),
            jam_score_regex: Regex::new(r#"ScoreBoard\.Game\(([^\)]+)\)\.Period\((\d+)\)\.Jam\((\d+)\)\.TeamJam\((\d+)\)\.JamScore"#).unwrap(),
            jam_skater_regex: Regex::new(r#"ScoreBoard\.Game\(([^\)]+)\)\.Period\((\d+)\)\.Jam\((\d+)\)\.TeamJam\((\d+)\)\.Fielding\(Jammer\)\.Skater"#).unwrap(),
            skater_name_regex: Regex::new(r#"ScoreBoard\.Game\(([^\)]+)\)\.Team\((\d+)\)\.Skater\(([^\)]+)\)\.Name"#).unwrap(),
        }));

        let mut receiver = scoreboard.get_receiver();

        let update_sender = socket_server.get_update_sender();
        socket_server.register_update_provider(&"JammerStats".to_string(), jammer_stats.clone()).await;

        tokio::task::spawn(async move {
            while let Ok(state_update) = receiver.recv().await {
                let mut locked_jammer_stats = jammer_stats.lock().await;

                let update_game_ids = locked_jammer_stats.process_state_update(state_update);

                debug!("{} games updated", update_game_ids.len());
                for update_game_id in update_game_ids {
                    let update = if let Some(s) = locked_jammer_stats.game_states.get(&update_game_id) {
                        s.clone()
                    } else {
                        continue;
                    };

                    debug!("Sending JammerStats update for game {}", update_game_id.clone());
                    if let Err(e) = update_sender.send(Update { game_id: update_game_id, data_type: "JammerStats".to_string(), update: json!(update.clone())}) {
                        error!("Error sending update on mpsc: {:?}", e);
                    }
                }
            }
        });

        scoreboard.register_topic("ScoreBoard.Game(*).Team(*).Skater(*).Name");
        scoreboard.register_topic("ScoreBoard.Game(*).Period(*).Jam(*).TeamJam(*).Fielding(Jammer).Skater");
        scoreboard.register_topic("ScoreBoard.Game(*).Period(*).Jam(*).TeamJam(*).JamScore");
    }

    fn process_state_update(&mut self, update: ScoreboardState) -> Vec<String> {
        debug!("Processing stats update for jammer stats");

        let stats_by_game = update.iter()
            .filter_map(|s| self.get_relevant_states(s))
            .fold(HashMap::new(), |mut map, match_info| {
                let game_id = match match_info.clone() {
                    Match::JammerScore(jammer_score) => jammer_score.game,
                    Match::JammerSkater(jammer_skater) => jammer_skater.game,
                    Match::SkaterName(skater_name) => skater_name.game,
                };
        
                if !map.contains_key(&game_id) {
                    map.insert(game_id.clone(), Vec::new());
                }
        
                map.get_mut(&game_id).unwrap().push(Box::new(match_info));
        
                map
            });
        
        for game_id in stats_by_game.keys() {
            let game_stats = stats_by_game.get(game_id).unwrap();

            let skater_names = game_stats.iter()
                .filter_map(|m| {
                    if let Match::SkaterName(s) = *m.clone() { 
                        Some(s) 
                    } else { 
                        None 
                    } 
                });

            struct JamInfo {
                jammer_id: String,
                score: u64,
            }

            let jam_stats = game_stats.iter()
                .filter_map(|m| {
                    let m = *m.clone();
                    if let Match::JammerScore(_) = m {
                        Some(m)
                    } else if let Match::JammerSkater(_) = m {
                        Some(m)
                    } else {
                        None
                    }
                })
                .fold(HashMap::new(), |mut map, m| {
                    let key = match m.clone() {
                        Match::JammerScore(jammer_score) => (jammer_score.jam, jammer_score.team),
                        Match::JammerSkater(skater) => (skater.jam, skater.team),
                        _ => panic!("Unexpected state type")
                    };

                    if !map.contains_key(&key) {
                        map.insert(key.clone(), JamInfo { jammer_id: "".to_string(), score: 0 });
                    }
                    let jam = map.get_mut(&key).unwrap();

                    match m.clone() {
                        Match::JammerScore(score) => jam.score = score.score,
                        Match::JammerSkater(skater) => jam.jammer_id = skater.skater_id.clone(),
                        _ => panic!("Unexpected state type")
                    };
                    
                    map
                });

            let jammer_stats: Vec<JammerInfo> = skater_names
                .map(|skater| {
                    let skater_jam_stats: Vec<(&u32, &JamInfo)> = jam_stats.iter()
                        .filter_map(|((jam_number, _), jam)| {
                            if jam.jammer_id.eq(&skater.skater_id) {
                                Some((jam_number, jam))
                            } else {
                                None
                            }
                        })
                        .collect();

                    debug!("Found {} jam stats for skater {} in game {}", skater_jam_stats.len(), skater.name, game_id);

                    JammerInfo {
                        name: skater.name.clone(),
                        team: skater.team,
                        jam_count: skater_jam_stats.len(),
                        total_score: skater_jam_stats.iter().fold(0, |score, (_, jam)| score + jam.score),
                        mean_net_per_jam: 0.0, /* TODO */
                        lead_count: 0, /* TODO */
                        mean_time_to_initial: 0.0, /* TODO */
                    }
                })
                .filter(|stats| stats.jam_count > 0)
                .collect();

            self.game_states.insert(game_id.clone(), JammerStatsStates { jammers: jammer_stats });
        }

        stats_by_game.keys().map(|k| k.clone()).collect()
    }

    fn get_relevant_states(&self, (key, value): (&String, &Value)) -> Option<Match> {
        if self.jam_score_regex.is_match(key) {
            self.jam_score_regex.captures(key).map(|c| {
                let (_, [game, _period, jam, team]) = c.extract();
                Match::JammerScore(
                    JammerScoreMatches {
                        game: game.to_string(),
                        jam: jam.parse::<u32>().unwrap(),
                        team: team.parse::<u8>().unwrap(),
                        score: value.as_u64().unwrap()
                    })
            })                    
        } else if self.jam_skater_regex.is_match(key) {
            self.jam_skater_regex.captures(key).map(|c| {
                let (_, [game, _period, jam, team]) = c.extract();

                Match::JammerSkater(
                    JammerSkaterMatches {
                        game: game.to_string(),
                        jam: jam.parse::<u32>().unwrap(),
                        team: team.parse::<u8>().unwrap(),
                        skater_id: value.as_str().unwrap().to_string(),
                    })
            })
        } else if self.skater_name_regex.is_match(key) {
            self.skater_name_regex.captures(key).map(|c| {
                let (_, [game, team, skater_id]) = c.extract();

                Match::SkaterName(
                    SkaterNameMatches {
                        game: game.to_string(),
                        team: team.parse::<u8>().unwrap(),
                        skater_id: skater_id.to_string(),
                        name: value.as_str().unwrap().to_string(),
                    })
            })
        } else {
            None
        }
    }
}

impl UpdateProvider for JammerStats {
    fn get_state(&self, game_id: &String) -> serde_json::Value {
        json!(self.game_states.get(game_id).unwrap().clone())
    }
}
