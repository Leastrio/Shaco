use std::{fmt, fmt::Display};

use serde_json::Value;

/// The Websocket connection returns LcuEvents
#[derive(Debug, Clone)]
pub struct LcuEvent {
    pub subscription_type: LcuSubscriptionType,
    pub data: Value,
    pub event_type: String,
}

/// LcuEvents first get deserialized to deserialize::DeEvent and then to LcuEvent
/// because the data formats are not directly deserializable by serde
impl From<deserialize::DeEvent> for LcuEvent {
    fn from(de_event: deserialize::DeEvent) -> LcuEvent {
        Self {
            subscription_type: de_event.subscription_type,
            data: de_event.data.data,
            event_type: de_event.data.event_type,
        }
    }
}

/// The Websocket events to subscribe to.
/// Look at the in-official documentation for event strings to subscribe to.
///
/// https://www.mingweisamuel.com/lcu-schema/tool/#/
///
/// /lol-gameflow/v1/gameflow-phase => LcuSubscriptionType::JsonApiEvent("lol-gameflow_v1_gameflow-phase".to_string())
#[derive(Debug, Clone)]
pub enum LcuSubscriptionType {
    AllJsonApiEvents,
    AllLcdsEvents,
    JsonApiEvent(String),
    LcdsEvent(String),
}
impl Display for LcuSubscriptionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LcuSubscriptionType::AllJsonApiEvents => f.write_str("OnJsonApiEvent"),
            LcuSubscriptionType::AllLcdsEvents => f.write_str("OnJsonApiEvent"),
            LcuSubscriptionType::JsonApiEvent(s) => f.write_str(&format!("OnJsonApiEvent_{}", s)),
            LcuSubscriptionType::LcdsEvent(s) => f.write_str(&format!("OnLcdsEvent_{}", s)),
        }
    }
}

/// Intermediate data-structures for deserializing LcuEvents
pub(crate) mod deserialize {
    use serde::{de, Deserialize, Deserializer};
    use serde_json::Value;

    use super::LcuSubscriptionType;

    /// Main intermediate data-structure
    #[derive(Deserialize, Debug)]
    pub struct DeEvent {
        _opcode: i64,
        pub(crate) subscription_type: LcuSubscriptionType,
        pub(crate) data: Data,
    }

    /// part of DeEvent
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Data {
        pub(crate) data: Value,
        pub(crate) event_type: String,
    }

    /// Custom deserializer to differentiate between the different subscription types
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
}
