mod scoreboard_connector;
mod socket_server;
mod cumulative_score;
mod penalties_by_type;
mod jammer_stats;
mod game_info;

use clap::Parser;
use simplelog::{CombinedLogger, TermLogger, Config, TerminalMode, ColorChoice};
use log::{info, LevelFilter};

use crate::{
    cumulative_score::CumulativeScore,
    game_info::GameInfo,
    jammer_stats::JammerStats,
    penalties_by_type::PenaltiesByType,
    scoreboard_connector::ScoreboardConnection,
    socket_server::SocketServer,
};

#[derive(Parser, Debug)]

struct CommandLineArguments {
    #[arg(short = 'u', long = "scoreboardUrl", default_value = "localhost:8000")]
    scoreboard_url: String,

    #[arg(short = 'p', long = "hostPort", default_value_t = 8001)]
    host_port: u16,

    #[arg(long = "logLevel", default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() {

    let arguments = CommandLineArguments::parse();

    let log_level = parse_log_level(arguments.log_level.as_str());
    CombinedLogger::init(
        vec![
            TermLogger::new(log_level, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        ]
    ).unwrap();

    info!("Connecting to scoreboard");

    let scoreboard_socket_url = format!("ws://{}/WS", arguments.scoreboard_url);

    let mut scoreboard_connection = ScoreboardConnection::new(scoreboard_socket_url).unwrap();

    info!("Starting API endpoints");
    let mut server = SocketServer::new();
    CumulativeScore::new(&mut scoreboard_connection, &mut server).await;
    PenaltiesByType::new(&mut scoreboard_connection, &mut server).await;
    JammerStats::new(&mut scoreboard_connection, &mut server).await;
    GameInfo::new(&mut scoreboard_connection, &mut server).await;

    server.listen(arguments.host_port).await;
}

fn parse_log_level(level: &str) -> LevelFilter {
    match level.to_ascii_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        "none" => LevelFilter::Off,
        _ => LevelFilter::Info
    }
}