use std::error::Error;
use std::fmt;
use std::fmt::Display;

/// Errors for the Websocket connection to the LCU API
#[derive(Debug, Clone)]
pub enum LcuWebsocketError {
    LcuNotAvailable(String),
    AuthError,
    SendError,
    Disconnected(String),
}

impl Error for LcuWebsocketError {}

impl Display for LcuWebsocketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LcuNotAvailable(s) => write!(f, "LCU API not available: {}", s),
            Self::AuthError => write!(f, "Authentication error"),
            Self::SendError => write!(f, "Error sending message"),
            Self::Disconnected(s) => write!(f, "Websocket disconnected: {}", s),
        }
    }
}

/// Errors that can occur when trying to get the Riot process information
#[derive(Debug, Clone)]
pub enum ProcessInfoError {
    ProcessNotAvailable,
    PortNotFound,
    AuthTokenNotFound,
}

impl Error for ProcessInfoError {}

impl Display for ProcessInfoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProcessNotAvailable => write!(f, "Riot/League client process could not be found"),
            Self::PortNotFound => write!(f, "API port could not be parsed from process arguments"),
            Self::AuthTokenNotFound => write!(
                f,
                "API auth token could not be parsed from process arguments"
            ),
        }
    }
}
