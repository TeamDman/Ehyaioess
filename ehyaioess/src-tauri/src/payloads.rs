use serde::{Serialize, Deserialize};
use ts_rs::TS;



#[derive(Debug, TS, Serialize, Deserialize, Clone)]
#[ts(export, export_to = "../src/lib/bindings/")]
pub struct ConversationTitleChangedEventPayload {
    #[ts(type="string")]
    pub conversation_id: uuid::Uuid,
    pub new_title: String,
}

#[derive(Debug, TS, Serialize, Deserialize, Clone)]
#[ts(export, export_to = "../src/lib/bindings/")]
pub struct ConversationMessagePayload {
    #[ts(type="\"system\" | \"user\" | \"assistant\"")]
    pub author: chatgpt::types::Role,
    pub content: String,
}

#[derive(Debug, TS, Serialize, Deserialize, Clone)]
#[ts(export, export_to = "../src/lib/bindings/")]
pub struct ConversationMessageAddedEventPayload {
    #[ts(type="string")]
    pub conversation_id: uuid::Uuid,
    #[ts(type="\"system\" | \"user\" | \"assistant\"")]
    pub author: chatgpt::types::Role,
    pub content: String,
}