mod scoreboard_connector;
mod socket_server;
mod cumulative_score;

use scoreboard_connector::ScoreboardConnection;
use socket_server::SocketServer;

fn main() {
    let mut scoreboard_connection = ScoreboardConnection::new("ws://192.168.86.33:8000/WS").unwrap();

    for topic in get_required_topics() {
        scoreboard_connection.register_topic(topic.as_str());
    }

    SocketServer::new();
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