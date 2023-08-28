use std::collections::HashMap;

use crate::scoreboard_connector::StateUpdate;

pub struct ScoreboardData {
    state: ScoreboardState,
}

impl ScoreboardData {
    pub fn new() -> ScoreboardData {
        ScoreboardData {
            state: ScoreboardState {
                version: String::from(""),
                game: Game {
                    clocks: HashMap::from([
                        (String::from("Period"), Clock { time: 0 })
                    ]),
                    periods: vec![
                        Period {
                            jams: vec![]
                        },
                        Period {
                            jams: vec![]
                        }
                    ],
                    teams: vec![
                        Team {
                            skaters: HashMap::new(),
                        },
                        Team {
                            skaters: HashMap::new(),
                        }
                    ]
                }
            }
        }
    }

    pub fn apply_update(mut self, update: StateUpdate) -> ScoreboardData {
        let state = update.state.as_object().unwrap();

        for key in state.keys() {
            match key.as_str() {
                "ScoreBoard.CurrentGame.Clock(Period).Time" => {
                    self.state.game.clocks.entry("Period".into()).and_modify(|e| e.time = state[key].as_i64().unwrap());
                }

                

                _ => { }
            }
        }

        self
    }
}

pub struct ScoreboardState {
    version: String,
    game: Game,
}

pub struct Clock {
    time: i64,
}

pub struct Game {
    clocks: HashMap<String, Clock>,
    teams: Vec<Team>,
    periods: Vec<Period>,
}

pub struct Period {
    jams: Vec<Jam>
}

pub struct Jam {
    team_jams: Vec<TeamJam>,
}

pub struct TeamJam {
    total_score: i32,
}

pub struct Team {
    skaters: HashMap<String, Skater>,
}

pub struct Skater {
    penalties: Vec<Penalty>,
}

pub struct Penalty {
    code: String,
}