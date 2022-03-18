use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{header, Certificate};
use sysinfo::{ProcessExt, System, SystemExt};

use crate::RESTClient;

type Error = Box<dyn std::error::Error>;

impl RESTClient {

    /// Create a new instance of the LCU REST wrapper
    pub fn new() -> Result<Self, Error> {
        let mut sys = System::new_all();
        sys.refresh_all();
        let process = find_process(&sys)?;
        let args = extract_info(process)?;
        let rest_token = encode_token(&args.0);
        let client = build_reqwest_client(&rest_token)?;
        Ok(Self {
            port: args.1,
            reqwest_client: client,
        })
    }

    /// Make a get request to the specified endpoint
    pub async fn get(&self, endpoint: &'static str) -> Result<serde_json::Value, reqwest::Error> {
        let req: serde_json::Value = self
            .reqwest_client
            .get(format!("https://127.0.0.1:{}{}", self.port, endpoint))
            .send()
            .await?
            .json()
            .await?;

        Ok(req)
    }

    /// Make a post request to the specified endpoint
    pub async fn post(
        &self,
        endpoint: &'static str,
        body: serde_json::Value,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let req: serde_json::Value = self
            .reqwest_client
            .post(format!("https://127.0.0.1:{}{}", self.port, endpoint))
            .json(&body)
            .send()
            .await?
            .json()
            .await?;

        Ok(req)
    }

    /// Make a put request to the specified endpoint
    pub async fn put(
        &self,
        endpoint: &'static str,
        body: serde_json::Value,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let req: serde_json::Value = self
            .reqwest_client
            .put(format!("https://127.0.0.1:{}{}", self.port, endpoint))
            .json(&body)
            .send()
            .await?
            .json()
            .await?;

        Ok(req)
    }
}

fn find_process(system: &System) -> Result<String, &'static str> {
    let mut res: Option<String> = None;
    for process in system.processes().values() {
        if process.name() == "LeagueClientUx.exe" {
            res = Some(process.cmd().join(" "));
            break;
        }
    }
    match res {
        Some(x) => Ok(x),
        None => Err("Could not find a running LCU process!"),
    }
}

fn extract_info(cmd_args: String) -> Result<(String, u32), &'static str> {
    lazy_static! {
        static ref PORT_RE: Regex = Regex::new(r"--app-port=([0-9]*)").unwrap();
        static ref TOKEN_RE: Regex = Regex::new(r"--remoting-auth-token=([\w-]*)").unwrap();
    }
    let port = match PORT_RE.captures(&cmd_args) {
        Some(x) => match x.get(1) {
            Some(y) => match y.as_str().parse::<u32>() {
                Ok(z) => z,
                Err(_) => return Err("Failed to parse port"),
            },
            None => return Err("No port found!"),
        },
        None => return Err("No port found!"),
    };

    let token = match TOKEN_RE.captures(&cmd_args) {
        Some(x) => match x.get(1) {
            Some(y) => y.as_str(),
            None => return Err("No authentication token found!"),
        },
        None => return Err("No authentication token found!"),
    };

    Ok((token.to_string(), port))
}

fn encode_token(remote_token: &str) -> String {
    let token = format!("riot:{}", remote_token);
    base64::encode(token)
}

fn build_reqwest_client(auth_token: &str) -> Result<reqwest::Client, Error> {
    let cert = Certificate::from_pem(include_bytes!("../riotgames.pem"))?;

    let mut headers = header::HeaderMap::new();
    let auth_header =
        header::HeaderValue::from_str(format!("Basic {}", auth_token).as_str()).unwrap();
    headers.insert("Authorization", auth_header);

    Ok(reqwest::ClientBuilder::new()
        .add_root_certificate(cert)
        .default_headers(headers)
        .build()?)
}
