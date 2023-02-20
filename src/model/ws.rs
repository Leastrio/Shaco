use std::error::Error;
use std::fmt::{Display, Formatter, Result};

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
pub struct Event {
    pub subscription_type: SubscriptionType,
    pub data: String,
    pub event_type: String,
}
impl<'a> From<deserialize::DeEvent<'a>> for Event {
    fn from(de_event: deserialize::DeEvent) -> Event {
        Self {
            subscription_type: de_event.subscription_type,
            data: de_event.data.data.get().to_string(),
            event_type: de_event.data.event_type,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SubscriptionType {
    AllJsonApiEvents,
    AllLcdsEvents,
    JsonApiEvent(String),
    LcdsEvent(String),
}
impl Display for SubscriptionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            SubscriptionType::AllJsonApiEvents => f.write_str("OnJsonApiEvent"),
            SubscriptionType::AllLcdsEvents => f.write_str("OnJsonApiEvent"),
            SubscriptionType::JsonApiEvent(s) => f.write_str(&format!("OnJsonApiEvent_{}", s)),
            SubscriptionType::LcdsEvent(s) => f.write_str(&format!("OnLcdsEvent_{}", s)),
        }
    }
}

pub(crate) mod deserialize {
    use super::SubscriptionType;
    use serde::{de, Deserialize, Deserializer};
    use serde_json::value::RawValue;

    impl<'de> Deserialize<'de> for SubscriptionType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;

            if s.starts_with("OnJsonApiEvent") {
                if s.len() > 14 {
                    Ok(SubscriptionType::JsonApiEvent(s[15..].to_string()))
                } else {
                    Ok(SubscriptionType::AllJsonApiEvents)
                }
            } else if s.starts_with("OnLcdsApiEvent") {
                if s.len() > 14 {
                    Ok(SubscriptionType::LcdsEvent(s[15..].to_string()))
                } else {
                    Ok(SubscriptionType::AllLcdsEvents)
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
    pub struct Data<'a> {
        #[serde(borrow)]
        pub(crate) data: &'a RawValue,
        pub(crate) event_type: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct DeEvent<'a> {
        _opcode: i64,
        pub(crate) subscription_type: SubscriptionType,
        #[serde(borrow)]
        pub(crate) data: Data<'a>,
    }
}
