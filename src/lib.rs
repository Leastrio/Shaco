//! # Shaco
//!
//! A LCU REST + WAMP api wrapper

use futures_util::stream::{SplitSink, SplitStream};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

pub mod ingame;
pub mod model;
pub mod rest;
mod utils;
pub mod ws;

pub struct RESTClient {
    port: u32,
    reqwest_client: reqwest::Client,
}

pub struct InGameClient {
    port: u32,
    reqwest_client: reqwest::Client,
}

pub struct WSClient {
    write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}
