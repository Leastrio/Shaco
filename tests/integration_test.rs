use std::time::Duration;

use futures_util::StreamExt;
use native_tls::TlsConnector;
use tokio::net::TcpStream;
use tokio::time::Instant;

use hyper::body::{Bytes, HttpBody};
use hyper::Uri;
use hyper::{Body, Client};
use hyper_tls::HttpsConnector;
use shaco::ingame::EventStream;
use shaco::model::ws::LcuSubscriptionType;
use shaco::ws::WSClient;

#[tokio::test]
async fn ingame_event_stream() {
    let mut event_stream = EventStream::new(None).unwrap();
    while let Some(event) = event_stream.next().await {
        println!("{:?}", event)
    }
}

#[tokio::test]
async fn lcu_event_stream() {
    let mut ws_client = WSClient::connect().await.unwrap();
    ws_client
        .subscribe(LcuSubscriptionType::AllJsonApiEvents)
        .await
        .unwrap();

    while let Some(event) = ws_client.next().await {
        println!("Event: {:?}", event);
    }
}