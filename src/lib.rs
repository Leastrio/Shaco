//! # Shaco
//!
//! A LCU REST + WAMP api wrapper

use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

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
    ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}
