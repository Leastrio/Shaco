//! # Shaco
//!
//! A LCU REST + WAMP api wrapper

pub mod rest;
// pub mod wamp;
pub mod ingame;
mod utils;
mod model;

pub struct RESTClient {
    port: u32,
    reqwest_client: reqwest::Client
}

pub struct InGameClient {
    port: u32,
    reqwest_client: reqwest::Client
}

// pub struct WAMPClient<'a> {
//     port: u32,
//     password: String,
//     wamp_client: Client<'a>
// }
