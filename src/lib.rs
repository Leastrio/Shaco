pub mod rest;
pub struct RESTClient {
    port: u32,
    reqwest_client: reqwest::Client,
}
