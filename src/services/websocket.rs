use std::collections::VecDeque;
use std::net::SocketAddr;
use futures_util::future::join_all;
use futures_util::SinkExt;
use futures_util::StreamExt;
use sea_orm::DbConn;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tracing::debug;
use tracing::info;
use crate::CONFIG;
use crate::types::nostr::event::Event;
use crate::types::nostr::filter::Filter;
use crate::types::nostr::protocol::Protocol;
use crate::types::nostr::protocol::ProtocolCategory;
use crate::types::nostr::protocol::ProtocolData;

//-////////////////////////////////////////////////////////////////////////////
//  Websocket Listener
//-////////////////////////////////////////////////////////////////////////////
pub async fn run_ws_listener(db: DbConn) {
    let listener = TcpListener::bind(CONFIG.server_addr).await.unwrap();

    info!("websocket listening at: '{}'", CONFIG.server_addr);

    while let Ok((ws_stream, client_addr)) = listener.accept().await {
        tokio::spawn(handle_connection(client_addr, ws_stream, db.clone()));
    }
}

//-////////////////////////////////////////////////////////////////////////////
//  Connection Handler
//-////////////////////////////////////////////////////////////////////////////

/// Handles received connection
async fn handle_connection(client: SocketAddr, stream: TcpStream, db: DbConn) {
    info!(" -- {} connected --", &client);
    let mut ws_stream = accept_async(stream).await.expect("Handshake error!");

    while let Some(msg) = ws_stream.next().await {
        match msg.unwrap() {
            Message::Text(text) => {
                let raw: VecDeque<ProtocolData> = serde_json::from_str(&text).unwrap();
                match Protocol::from_data(raw) {
                    Err(err) => {ws_stream.send(Message::Text(serde_json::to_string(&(ProtocolCategory::NOTICE, err)).unwrap())).await.unwrap();},
                    Ok(protocol) => match protocol {
                        Protocol::Event(event) => {
                            debug!("{} --> {:?}", &client, &event);
                            handle_event(event, &db).await;
                        },
                        Protocol::Request{sub_id, filters} => {
                            debug!("{} --> Request: {:?}, {:?}", &client, &sub_id, &filters);
                            let events = handle_request(&sub_id, filters, &db).await;
                            debug!("{} <-- Events: {:?}", &client, &events);
                            for event in events {
                                ws_stream.send(Message::Text(serde_json::to_string(&(ProtocolCategory::EVENT, sub_id.clone(), event)).unwrap())).await.unwrap();
                            }
                        },
                        Protocol::Close{sub_id} => handle_close(&sub_id).await,
                    },
                };
            },
            Message::Close(_) => (),
            _ => (),
        }
    };

    info!(" -- {} disconnected --", &client);
}

//-////////////////////////////////////////////////////////////////////////////
//  Message Handler
//-////////////////////////////////////////////////////////////////////////////
async fn handle_event(event: Event, db: &DbConn) {
    event.insert(&db).await;
}

async fn handle_request(_sub_id: &String, filters: Vec<Filter>, db: &DbConn) -> Vec<Event> {
    let events = join_all(filters.into_iter().map(|filter| filter.find_event(db))).await;
    events.into_iter()
        .flatten()
        .collect()
}

async fn handle_close(_sub_id: &String) {

}
//-////////////////////////////////////////////////////////////////////////////
//
//-////////////////////////////////////////////////////////////////////////////
