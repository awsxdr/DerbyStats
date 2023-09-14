use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use log::{debug, error};
use regex::Regex;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use crate::{socket_server::{UpdateProvider, Update, SocketServer}, scoreboard_connector::{ScoreboardConnection, ScoreboardState}};

#[derive(Serialize, Deserialize, Clone)]
struct Team {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "color")]
    pub color: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Game {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "isCurrent")]
    pub is_current: bool,

    #[serde(rename = "homeTeam")]
    pub home_team: Team,

    #[serde(rename = "awayTeam")]
    pub away_team: Team,
}

impl Game {
    pub fn new(id: String) -> Game {
        Game {
            id,
            is_current: false,
            home_team: Team {
                name: "".to_string(),
                color: "".to_string(),
            },
            away_team: Team {
                name: "".to_string(),
                color: "".to_string(),
            },
        }
    }
}

pub struct GameInfo {
    games: Vec<Game>,
    current_game_regex: Regex,
    team_regex: Regex,
}

#[derive(Clone)]
enum Match {
    CurrentGame(CurrentGameMatches),
    Team(TeamMatches),
}

#[derive(Clone)]
struct CurrentGameMatches {
    game_id: String,
}

#[derive(Clone)]
struct TeamMatches {
    game_id: String,
    team_id: u8,
    property_name: String,
    value: String,
}

impl GameInfo {
    pub async fn new(scoreboard: &mut ScoreboardConnection, socket_server: &mut SocketServer) {
        let game_info = Arc::new(Mutex::new(GameInfo { 
            games: Vec::new(),
            current_game_regex: Regex::new(r#"ScoreBoard\.CurrentGame\.Game"#).unwrap(),
            team_regex: Regex::new(r#"ScoreBoard\.Game\(([^\)]+)\)\.Team\((\d+)\)\.(.+)"#).unwrap(),
        }));

        let mut receiver = scoreboard.get_receiver();

        let update_sender = socket_server.get_update_sender();
        socket_server.register_update_provider(&"Games".to_string(), game_info.clone()).await;

        tokio::task::spawn(async move {
            while let Ok(state_update) = receiver.recv().await {
                let mut locked_games = game_info.lock().await;

                locked_games.process_state_update(state_update);

                let update = Update { 
                    game_id: "*".to_string(),
                    data_type: "Games".to_string(),
                    update: json!(locked_games.games)
                };

                debug!("Sending Games update");
                if let Err(e) = update_sender.send(update) {
                    error!("Error sending update on mpsc: {:?}", e);
                }
            }
        });

        scoreboard.register_topic("ScoreBoard.CurrentGame.Game");
        scoreboard.register_topic("ScoreBoard.Game(*).Team(*).Name");
        scoreboard.register_topic("ScoreBoard.Game(*).Team(*).UniformColor");
    }

    fn process_state_update(&mut self, update: ScoreboardState) {
        debug!("Processing stats update for games");

        let games = update.iter()
            .filter_map(|s| self.get_relevant_states(s))
            .fold(HashMap::new(), |mut map, match_info| {
                let game_id = match match_info.clone() {
                    Match::CurrentGame(current_game) => current_game.game_id,
                    Match::Team(team) => team.game_id,
                };

                if !map.contains_key(&game_id) {
                    map.insert(game_id.clone(), Game::new(game_id.clone()));
                }

                let game = map.get_mut(&game_id).unwrap();

                match match_info {
                    Match::CurrentGame(current) => {
                        game.is_current = true;
                        game.id = current.game_id;
                    },
                    Match::Team(team_match) => {
                        let team = if team_match.team_id == 1 { &mut game.home_team } else { &mut game.away_team };
                        match team_match.property_name.as_str() {
                            "Name" => {
                                team.name = team_match.value;
                            },
                            "UniformColor" => {
                                team.color = team_match.value;
                            },
                            _ => { }
                        };
                    }
                }

                map
            });

        self.games = games.values().cloned().collect();
    }

    fn get_relevant_states(&self, (key, value): (&String, &Value)) -> Option<Match> {
        if self.current_game_regex.is_match(key) {
            debug!("Current game state update: {}", value.to_string());
            Some(Match::CurrentGame(CurrentGameMatches {
                game_id: value.as_str().unwrap().to_string(),
            }))
        } else if self.team_regex.is_match(key) {
            self.team_regex.captures(key).map(|c| {
                let (_, [game_id, team_id, property_name]) = c.extract();

                Match::Team(TeamMatches { 
                    game_id: game_id.to_string(),
                    team_id: team_id.parse::<u8>().unwrap(),
                    property_name: property_name.to_string(), 
                    value: value.as_str().unwrap().to_string(),
                })
            })
        } else {
            None
        }
    }
}

impl UpdateProvider for GameInfo {
    fn get_state(&self, _game_id: &String) -> serde_json::Value {
        json!(self.games)
    }
}
