mod scoreboard_connector;
mod scoreboard_data;

use scoreboard_connector::ScoreboardConnection;

use crate::scoreboard_data::ScoreboardData;

fn main() {
    let mut scoreboard_connection = ScoreboardConnection::new("ws://192.168.86.33:8000/WS").unwrap();

    for topic in get_required_topics() {
        scoreboard_connection.register_topic(topic.as_str());
    }

    let mut scoreboard_data = ScoreboardData::new();

    for message in scoreboard_connection {
        scoreboard_data = scoreboard_data.apply_update(message);
    }
}

fn get_required_topics() -> Vec<String> {
    vec![
        "ScoreBoard.Version(release)",
        "ScoreBoard.CurrentGame.Clock(Period)",
        "ScoreBoard.CurrentGame.CurrentPeriodNumber",
        "ScoreBoard.CurrentGame.Period(*).Jam(*).TeamJam(*).TotalScore",
        "ScoreBoard.CurrentGame.Team(*).Skater(*).Penalty(*)",
    ].into_iter().map(String::from).collect()
}