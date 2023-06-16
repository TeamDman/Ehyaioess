// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use chatgpt::prelude::ChatGPT;
use std::collections::HashMap;
use tauri::{async_runtime::RwLock, Manager, State};

use crate::models::{
    Conversation, ConversationManager, ConversationMessage, ConversationTitleChangedEvent, MyError,
};

#[tauri::command(rename_all = "snake_case")]
pub async fn list_conversations(
    conversation_manager: State<'_, RwLock<ConversationManager>>,
) -> Result<HashMap<uuid::Uuid, Conversation>, String> {
    Ok(conversation_manager.read().await.conversations.clone())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn new_conversation(
    conversation_manager: State<'_, RwLock<ConversationManager>>,
    config: State<'_, crate::config::Config>,
) -> Result<Conversation, MyError> {
    let mut mgr = conversation_manager.write().await;
    let conv = Conversation::new();

    mgr.conversations.insert(conv.id, conv.clone());
    mgr.write_to_disk(&config.conversation_history_save_path)
        .map_err(|_| MyError::ConversationWriteToDiskFail)?;

    Ok(conv)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn set_conversation_title(
    conversation_manager: State<'_, RwLock<ConversationManager>>,
    config: State<'_, crate::config::Config>,
    app_handle: tauri::AppHandle,
    id: &str,
    new_title: &str,
) -> Result<(), MyError> {
    let conversation_id = uuid::Uuid::parse_str(id).map_err(|_| MyError::UUIDParseFail)?;
    let mut mgr = conversation_manager.write().await;
    let conv = mgr
        .conversations
        .get_mut(&conversation_id)
        .ok_or(MyError::FindByIDFail)?;
    if conv.title == new_title {
        return Ok(());
    }

    conv.title = new_title.to_string();
    mgr.write_to_disk(&config.conversation_history_save_path)
        .map_err(|_| MyError::ConversationWriteToDiskFail)?;

    // Drop the lock before emitting events.
    drop(mgr);

    app_handle
        .emit_all(
            "conversation_title_changed",
            ConversationTitleChangedEvent {
                conversation_id,
                new_title: new_title.to_string(),
            },
        )
        .map_err(|_| MyError::EmitFail)?;

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn new_message(
    app_handle: tauri::AppHandle,
    conversation_manager: State<'_, RwLock<ConversationManager>>,
    conversation_id: &str,
    content: &str,
) -> Result<ConversationMessage, MyError> {
    let mut mgr = conversation_manager.write().await;
    let conv = mgr
        .conversations
        .get_mut(&uuid::Uuid::parse_str(conversation_id).unwrap())
        .ok_or(MyError::UUIDParseFail)?;
    conv.history.push(ConversationMessage {
        id: uuid::Uuid::new_v4(),
        author: chatgpt::types::Role::User,
        content: content.to_string(),
    });
    let msg = conv.history.last().unwrap();
    app_handle
        .emit_all("conversation_message_added", msg.clone())
        .map_err(|_| MyError::EmitFail)?;
    Ok(msg.clone())
}

#[tauri::command]
pub async fn greet(chatgpt: State<'_, ChatGPT>, name: &str) -> Result<String, String> {
    let prompt = format!("Hello from {}!", name);
    let response_result = chatgpt.send_message(prompt).await;
    let response = match response_result {
        Ok(response) => response.message().content.clone(),
        Err(e) => return Err(e.to_string()), // if there's an error sending the message, return it
    };

    Ok(response) // if everything is okay, return the content
}
