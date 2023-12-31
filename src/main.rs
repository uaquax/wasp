use std::{env::consts::OS, process::Command};

use futures_util::{SinkExt, StreamExt, TryFutureExt};
use serde_json::{json, to_string};
use warp::{
    filters::ws::{Message, WebSocket},
    Filter,
};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let routes = warp::path("terminal")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(move |socket| message(socket)));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn message(ws: WebSocket) {
    let (mut tx, mut rx) = ws.split();

    while let Some(res) = rx.next().await {
        match res {
            Ok(msg) => {
                let command = String::from_utf8_lossy(msg.as_bytes()).to_string();
                if command == "PING" {
                    continue;
                }

                if command == "%SYS_INFO%" {
                    let something = json!({"os": "Win11"}).to_string();
                    tx.send(Message::text(something))
                        .unwrap_or_else(|e| eprintln!("websocket send error: {}", e))
                        .await;
                }

                println!("Received command: {:?}", command);

                let result;
                if OS == "windows" {
                    result = Command::new("cmd")
                        .args(&["/C", command.as_str()])
                        .output()
                        .unwrap();
                } else {
                    result = Command::new("sh")
                        .args(&["-c", command.as_str()])
                        .output()
                        .unwrap();
                }
                let txt = String::from_utf8_lossy(&result.stdout).to_string();
                println!("output: {}", txt);

                tx.send(Message::text(txt))
                    .unwrap_or_else(|e| eprintln!("websocket send error: {}", e))
                    .await;
            }
            Err(e) => {
                eprintln!("websocket error: {:?}", e);
            }
        }
    }
}
