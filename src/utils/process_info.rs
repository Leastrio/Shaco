use base64::{engine::general_purpose, Engine};
use sysinfo::{ProcessExt, System, SystemExt};

#[cfg(target_os = "windows")]
const TARGET_PROCESS: &str = "LeagueClientUx.exe";
#[cfg(target_os = "linux")]
const TARGET_PROCESS: &str = "LeagueClientUx.";
#[cfg(target_os = "macos")]
const TARGET_PROCESS: &str = "LeagueClientUx";

pub fn get_auth_info() -> Result<(String, u16), &'static str> {
    let mut sys = System::new_all();
    sys.refresh_processes();

    let args = sys
        .processes()
        .values()
        .find(|p| p.name() == TARGET_PROCESS)
        .map(|p| p.cmd())
        .ok_or("no LoL process")?;

    let port = args
        .into_iter()
        .find(|arg| arg.starts_with("--app-port="))
        .map(|arg| arg[11..].parse::<u16>().ok())
        .flatten()
        .ok_or("no app_port")?;
    let auth_token = args
        .into_iter()
        .find(|arg| arg.starts_with("--remoting-auth-token="))
        .map(|arg| arg[22..].to_string())
        .ok_or("no remoting_auth_token")?;

    Ok((
        general_purpose::STANDARD.encode(format!("riot:{}", auth_token)),
        port,
    ))
}
