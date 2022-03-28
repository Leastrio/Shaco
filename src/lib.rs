//! # Shaco
//!
//! A LCU REST + WAMP api wrapper

use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream};

pub mod rest;
pub mod ws;
pub mod ingame;
pub mod model;
mod utils;

pub struct RESTClient {
    port: u32,
    reqwest_client: reqwest::Client,
}

pub struct InGameClient {
    port: u32,
    reqwest_client: reqwest::Client,
}

pub struct WSClient {
    ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>
    
}
