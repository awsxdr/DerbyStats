mod scoreboard_connector;
mod socket_server;
mod cumulative_score;
mod penalties_by_type;

use cumulative_score::CumulativeScore;
use scoreboard_connector::ScoreboardConnection;
use simplelog::{CombinedLogger, TermLogger, Config, TerminalMode, ColorChoice};
use socket_server::SocketServer;
use log::{info, LevelFilter};

use crate::penalties_by_type::PenaltiesByType;

#[tokio::main]
async fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        ]
    ).unwrap();

    info!("Connecting to scoreboard");
    let mut scoreboard_connection = ScoreboardConnection::new("ws://scoreboard/WS/").unwrap();

    info!("Starting API endpoints");
    let mut server = SocketServer::new();
    CumulativeScore::new(&mut scoreboard_connection, &mut server).await;
    PenaltiesByType::new(&mut scoreboard_connection, &mut server).await;

    server.listen().await;
}