use lazy_static::lazy_static;
use regex::Regex;
use sysinfo::{ProcessExt, System, SystemExt};

#[cfg(target_os = "windows")]
const TARGET_PROCESS: &str = "LeagueClientUx.exe";
#[cfg(target_os = "linux")]
const TARGET_PROCESS: &str = "LeagueClientUx.";
#[cfg(target_os = "macos")]
const TARGET_PROCESS: &str = "LeagueClientUx";

pub fn find_process(system: &System) -> Result<String, &'static str> {
    match system
    .processes()
    .values()
    .find(|process| process.name() == TARGET_PROCESS)
    .map(|process| process.cmd().join(" ")) {
        Some(x) => Ok(x),
        None => Err("Could not find a running LCU process!"),
    }
}

pub fn extract_info(cmd_args: String) -> Result<(String, u32), &'static str> {
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

pub fn encode_token(remote_token: &str) -> String {
    let token = format!("riot:{}", remote_token);
    base64::encode(token)
}
