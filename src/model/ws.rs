use std::error::Error;
use std::fmt::{Display, Formatter, Result};

use serde_json::Value;

#[derive(Debug, Clone)]
pub enum LcuWebsocketError {
    LcuNotAvailable,
    AuthError,
    SendError,
    Disconnected,
}

impl Error for LcuWebsocketError {}

impl Display for LcuWebsocketError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::LcuNotAvailable => write!(f, "LCU API not available"),
            Self::AuthError => write!(f, "Authentication error"),
            Self::SendError => write!(f, "Error sending message"),
            Self::Disconnected => write!(f, "Websocket disconnected"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LcuEvent {
    pub subscription_type: LcuSubscriptionType,
    pub data: Value,
    pub event_type: String,
}
impl From<deserialize::DeEvent> for LcuEvent {
    fn from(de_event: deserialize::DeEvent) -> LcuEvent {
        Self {
            subscription_type: de_event.subscription_type,
            data: de_event.data.data,
            event_type: de_event.data.event_type,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LcuSubscriptionType {
    AllJsonApiEvents,
    AllLcdsEvents,
    JsonApiEvent(String),
    LcdsEvent(String),
}
impl Display for LcuSubscriptionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            LcuSubscriptionType::AllJsonApiEvents => f.write_str("OnJsonApiEvent"),
            LcuSubscriptionType::AllLcdsEvents => f.write_str("OnJsonApiEvent"),
            LcuSubscriptionType::JsonApiEvent(s) => f.write_str(&format!("OnJsonApiEvent_{}", s)),
            LcuSubscriptionType::LcdsEvent(s) => f.write_str(&format!("OnLcdsEvent_{}", s)),
        }
    }
}

pub(crate) mod deserialize {
    use serde::{de, Deserialize, Deserializer};
    use serde_json::Value;

    use super::LcuSubscriptionType;

    impl<'de> Deserialize<'de> for LcuSubscriptionType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;

            if s.starts_with("OnJsonApiEvent") {
                if s.len() > 14 {
                    Ok(LcuSubscriptionType::JsonApiEvent(s[15..].to_string()))
                } else {
                    Ok(LcuSubscriptionType::AllJsonApiEvents)
                }
            } else if s.starts_with("OnLcdsApiEvent") {
                if s.len() > 14 {
                    Ok(LcuSubscriptionType::LcdsEvent(s[15..].to_string()))
                } else {
                    Ok(LcuSubscriptionType::AllLcdsEvents)
                }
            } else {
                Err(de::Error::custom(format!(
                    "Unknown SubscriptionType: {}",
                    s
                )))
            }
        }
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Data {
        pub(crate) data: Value,
        pub(crate) event_type: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct DeEvent {
        _opcode: i64,
        pub(crate) subscription_type: LcuSubscriptionType,
        pub(crate) data: Data,
    }
}
