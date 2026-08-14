use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum EventType {
    #[serde(rename = "QUEST_FULFILLED")]
    QuestFulfilled,
    #[serde(untagged)]
    Other(String),
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::QuestFulfilled => write!(f, "QUEST_FULFILLED"),
            EventType::Other(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EvaluateRequest {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub event: EventType,
}
