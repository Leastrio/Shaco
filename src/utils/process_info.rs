use base64::{engine::general_purpose, Engine};
use sysinfo::{Process, ProcessRefreshKind, RefreshKind, System};

use crate::error::ProcessInfoError;

#[cfg(target_os = "windows")]
const TARGET_PROCESS: &str = "LeagueClientUx.exe";
#[cfg(target_os = "linux")]
const TARGET_PROCESS: &str = "LeagueClientUx.";
#[cfg(target_os = "macos")]
const TARGET_PROCESS: &str = "LeagueClientUx";

pub(crate) fn get_auth_info() -> Result<(String, String), ProcessInfoError> {
    let sys = System::new_with_specifics(
        RefreshKind::new()
            .with_processes(ProcessRefreshKind::new().with_cmd(sysinfo::UpdateKind::Always)),
    );

    let command = sys
        .processes()
        .values()
        .find(|p| p.name() == TARGET_PROCESS)
        .map(Process::cmd)
        .ok_or(ProcessInfoError::ProcessNotAvailable)?;

    let port = command
        .iter()
        .find_map(|arg| arg.strip_prefix("--app-port="))
        .ok_or(ProcessInfoError::PortNotFound)?;
    let auth_token = command
        .iter()
        .find_map(|arg| arg.strip_prefix("--remoting-auth-token="))
        .ok_or(ProcessInfoError::AuthTokenNotFound)?;

    Ok((
        general_purpose::STANDARD.encode(format!("riot:{}", auth_token)),
        port.to_string(),
    ))
}
