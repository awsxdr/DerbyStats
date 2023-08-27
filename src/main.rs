mod scoreboard_connector;
use scoreboard_connector::ScoreboardConnection;

fn main() {
    println!("Hello, world!");

    let mut scoreboard_connection = ScoreboardConnection::new("ws://192.168.0.145:8000/WS").unwrap();
    scoreboard_connection.register_topic("Scoreboard.CurrentGame.Clock(Period)");
    scoreboard_connection.register_topic("Scoreboard.CurrentGame.CurrentPeriodNumber");

    for _message in scoreboard_connection {
        
    }
}
