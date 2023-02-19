use serde_json::Value;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum WebsocketError {
    DISCONNECTED,
}
impl Display for WebsocketError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            WebsocketError::DISCONNECTED => write!(f, "Websocket disconnected!"),
        }
    }
}
impl Error for WebsocketError {}

#[derive(Debug)]
pub struct Event {
    subscription_type: SubscriptionType,
    data: Value,
    event_type: String,
}
impl From<deserialize::DeEvent> for Event {
    fn from(de_event: deserialize::DeEvent) -> Event {
        Self {
            subscription_type: de_event.subscription_type,
            data: de_event.data.data,
            event_type: de_event.data.event_type,
        }
    }
}

#[derive(Debug)]
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
    use serde_json::Value;

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
    pub(crate) struct Data {
        pub(crate) data: Value,
        pub(crate) event_type: String,
    }

    #[derive(Deserialize, Debug)]
    pub(crate) struct DeEvent {
        _opcode: i64,
        pub(crate) subscription_type: SubscriptionType,
        pub(crate) data: Data,
    }
}
