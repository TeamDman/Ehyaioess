use serde::{Serialize, Deserialize};
use crate::models::{ChatRole, MyError};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub struct ConversationTitleChangedEventPayload {
    pub conversation_id: uuid::Uuid,
    pub new_title: String,
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
    ConversationMessageAddedEventPayload(ConversationMessageAddedEventPayload),
}

#[tauri::command]
#[specta::specta]
pub fn wrap_event_payloads(_bruh: WrapType) -> Result<(), MyError> {
    Ok(())
} 