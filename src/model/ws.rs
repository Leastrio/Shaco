use std::fmt::{self, Display};

use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;

/// The Websocket connection returns LcuEvents
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcuEvent {
    _opcode: u8,
    pub subscription_type: LcuSubscriptionType,
    pub payload: LcuEventData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcuEventData {
    pub data: Value,
    pub event_type: EventType,
    pub uri: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    Create,
    Update,
    Delete,
}

/// The Websocket events to subscribe to.
/// Look at the in-official documentation for event strings to subscribe to.
///
/// <https://www.mingweisamuel.com/lcu-schema/tool/#/>
///
/// e.g.: [LcuSubscriptionType::JsonApiEvent]\("/lol-gameflow/v1/gameflow-phase".to_string())
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
            LcuSubscriptionType::JsonApiEvent(s) => f.write_str(&format!(
                "OnJsonApiEvent_{}",
                s.trim_start_matches('/').replace('/', "_")
            )),
            LcuSubscriptionType::LcdsEvent(s) => f.write_str(&format!(
                "OnLcdsEvent_{}",
                s.trim_start_matches('/').replace('/', "_")
            )),
        }
    }
}

impl Serialize for LcuSubscriptionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
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
