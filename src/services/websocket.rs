use std::net::SocketAddr;
use futures_util::StreamExt;
use futures_util::TryStreamExt;
use futures::future;
use sea_orm::DbConn;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use crate::types::nostr::protocol::ProtocolData;
use crate::types::nostr::protocol::Protocol;

use crate::CONFIG;

pub async fn run_ws_listener(db: DbConn) {
    let listener = TcpListener::bind(CONFIG.server_addr).await.unwrap();

    println!("websocket listening at: '{}'", CONFIG.server_addr);

    while let Ok((ws_stream, client_addr)) = listener.accept().await {
        tokio::spawn(handle_connection(ws_stream, client_addr, db.clone()));
    }
}

/// Handles received connection
async fn handle_connection(stream: TcpStream, client_addr: SocketAddr, db: DbConn) {
    println!(" -- {} connected --", &client_addr);
    let ws_stream = accept_async(stream).await.expect("Handshake error!");

    let (_tx, rx) = ws_stream.split();

    rx.try_for_each(|req| {
        // println!("rx: {:?}", req);
        match req {
            Message::Text(text) => {
                println!("------------------------------------");
                println!("text: {:?}", text);
                let raw: Vec<ProtocolData> = serde_json::from_str(&text).unwrap();
                let req = Protocol::from_data(raw).unwrap();
            },
            Message::Close(_) => (),
            _ => (),
        }
        future::ok(())
    }).await.unwrap();

    println!(" -- {} disconnected --", &client_addr);
}

/// Handles client post request
async fn handle_post() {

}

/// Handles client fetch request
fn handle_fetch() {

}
