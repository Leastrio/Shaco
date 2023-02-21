use base64::{Engine, engine::general_purpose};
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
    system
        .processes()
        .values()
        .find(|process| process.name() == TARGET_PROCESS)
        .map(|process| process.cmd().join(" "))
        .ok_or("Could not find a running LCU process!")
}

lazy_static! {
    static ref PORT_RE: Regex = Regex::new(r"--app-port=([0-9]*)").unwrap();
    static ref TOKEN_RE: Regex = Regex::new(r"--remoting-auth-token=([\w-]*)").unwrap();
}

pub fn extract_info(cmd_args: String) -> Result<(String, u32), &'static str> {
    let port: u32 = PORT_RE
        .captures(&cmd_args)
        .map_or(Err("No port found!"), |value| {
            value.get(1).map_or(Err("No port found!"), |port| {
                port.as_str()
                    .parse()
                    .map_or(Err("Failed to parse port"), |port| Ok(port))
            })
        })?;

    let token =
        TOKEN_RE
            .captures(&cmd_args)
            .map_or(Err("No authentication token found!"), |value| {
                value
                    .get(1)
                    .map_or(Err("No authentication token found!"), |token| {
                        Ok(token.as_str())
                    })
            })?;

    Ok((token.to_string(), port))
}

pub fn encode_token(remote_token: &str) -> String {
    general_purpose::STANDARD.encode(&format!("riot:{}", remote_token))
}
