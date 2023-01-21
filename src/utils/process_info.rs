use lazy_static::lazy_static;
use regex::Regex;
use sysinfo::{ProcessExt, System, SystemExt};

pub fn find_process(system: &System) -> Result<String, &'static str> {
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
    base64::encode(format!("riot:{}", remote_token))
}
