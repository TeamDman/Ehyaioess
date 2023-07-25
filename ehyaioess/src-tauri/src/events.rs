use serde::{Serialize, Deserialize};
use crate::models::{ChatRole, MyError};
use specta::Type;



#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub struct ConversationTitleChangedEventPayload {
    pub conversation_id: uuid::Uuid,
    pub new_title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub struct ConversationMessagePayload {
    pub author: ChatRole,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub struct ConversationMessageAddedEventPayload {
    pub conversation_id: uuid::Uuid,
    pub author: ChatRole,
    pub content: String,
}

#[derive(Type, Serialize, Deserialize, Debug, Clone)]
pub enum WrapType {
    None,
    ConversationTitleChangedEventPayload(ConversationTitleChangedEventPayload),
    ConversationMessagePayload(ConversationMessagePayload),
}

#[tauri::command]
#[specta::specta]
pub fn wrap_event_payloads(_bruh: WrapType) -> Result<(), MyError> {
    Ok(())
} 

// #[derive(Debug, Serialize, Deserialize, Clone, Type)]
// #[ts(export, export_to = "../src/lib/bindings/")]
// pub struct ConversationEventRecordPayload {
//     #[ts(type="string")]
//     pub id: uuid::Uuid,
//     #[ts(type="string")]
//     pub conversation_id: Uuid,
//     pub timestamp: i64,
//     pub event: ConversationEvent,
// }

// #[derive(Debug, Serialize, Deserialize, Clone, Type)]
// #[ts(export, export_to = "../src/lib/bindings/")]
// pub struct ConversationPayload {
//     pub id: uuid::Uuid,
//     pub history: Vec<ConversationEventRecordPayload>,
// }