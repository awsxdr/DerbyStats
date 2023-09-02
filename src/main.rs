mod scoreboard_connector;
mod socket_server;
mod cumulative_score;

use cumulative_score::CumulativeScore;
use hyper::{service::{make_service_fn, service_fn}, Body, Request, Response, Result, Server, StatusCode};
use scoreboard_connector::ScoreboardConnection;
use socket_server::SocketServer;

#[tokio::main]
async fn main() {
    println!("Connecting to scoreboard");
    let mut scoreboard_connection = ScoreboardConnection::new("ws://192.168.86.33:8000/WS/").unwrap();

    println!("Registering topics");
    for topic in get_required_topics() {
        scoreboard_connection.register_topic(topic.as_str());
    }

    println!("Starting API endpoints");
    let server = SocketServer::new();
    CumulativeScore::new(scoreboard_connection, server);

    println!("Creating web sever");
    let make_service = make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(handle_web_request))
    });

    let address = "0.0.0.0:8001".parse().unwrap();
    let http_server = Server::bind(&address).serve(make_service);

    println!("Listening at http://{}/", address);
    let _ = http_server.await;
}

async fn handle_web_request(request: Request<Body>) -> Result<Response<Body>> {
    match (request.method(), request.uri().path()) {
        _ => send_file(request).await
    }
}

async fn send_file(request: Request<Body>) -> Result<Response<Body>> {
    let ui_path = std::env::current_exe().unwrap().parent().unwrap().join("ui");

    let serve_result = hyper_staticfile::resolve(&ui_path, &request).await;

    Ok(serve_result.map(|r| hyper_staticfile::ResponseBuilder::new()
        .request(&request)
        .build(r)
        .unwrap())
        .or_else(|_| Response::builder().status(StatusCode::NOT_FOUND).body(Body::empty()))
        .unwrap())
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