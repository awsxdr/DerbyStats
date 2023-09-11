mod scoreboard_connector;
mod socket_server;
mod cumulative_score;
mod penalties_by_type;

use clap::Parser;
use cumulative_score::CumulativeScore;
use scoreboard_connector::ScoreboardConnection;
use simplelog::{CombinedLogger, TermLogger, Config, TerminalMode, ColorChoice};
use socket_server::SocketServer;
use log::{info, LevelFilter};

use crate::penalties_by_type::PenaltiesByType;

#[derive(Parser, Debug)]
struct CommandLineArguments {
    #[arg(short = 'U', long = "scoreboardUrl")]
    scoreboard_url: String,

    #[arg(short = 'p', long = "hostPort", default_value_t = 8001)]
    host_port: u16,
}

#[tokio::main]
async fn main() {

    let arguments = CommandLineArguments::parse();

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        ]
    ).unwrap();

    info!("Connecting to scoreboard");

    let scoreboard_socket_url = format!("ws://{}/WS", arguments.scoreboard_url);

    let mut scoreboard_connection = ScoreboardConnection::new(scoreboard_socket_url).unwrap();

    info!("Starting API endpoints");
    let mut server = SocketServer::new();
    CumulativeScore::new(&mut scoreboard_connection, &mut server).await;
    PenaltiesByType::new(&mut scoreboard_connection, &mut server).await;

    server.listen(arguments.host_port).await;
}