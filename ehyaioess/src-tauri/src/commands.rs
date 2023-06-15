// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use chatgpt::prelude::ChatGPT;
use std::collections::HashMap;
use tauri::{async_runtime::RwLock, Manager, State};

use crate::models::{Conversation, ConversationManager, ConversationTitleChangedEvent};

#[tauri::command(rename_all = "snake_case")]
pub async fn list_conversations(
    conversation_manager: State<'_, RwLock<ConversationManager>>,
) -> Result<HashMap<uuid::Uuid, Conversation>, String> {
    Ok(conversation_manager.read().await.conversations.clone())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn new_conversation(
    conversation_manager: State<'_, RwLock<ConversationManager>>,
) -> Result<Conversation, ()> {
    let mut mgr = conversation_manager.write().await;
    let conv = mgr.new_conversation();
    Ok(conv.clone())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn set_conversation_title(
    conversation_manager: State<'_, RwLock<ConversationManager>>,
    app_handle: tauri::AppHandle,
    id: &str,
    new_title: &str,
) -> Result<(), ()> {
    let conversation_id = match uuid::Uuid::parse_str(id) {
        Ok(id) => id,
        Err(_) => return Err(()),
    };
    let mut mgr = conversation_manager.write().await;
    let conv = match mgr.conversations.get_mut(&conversation_id) {
        Some(conv) => conv,
        None => return Err(()),
    };
    conv.title = new_title.to_string();
    match app_handle.emit_all(
        "conversation_title_changed",
        ConversationTitleChangedEvent {
            conversation_id: conv.id.clone(),
            new_title: conv.title.clone(),
        },
    ) {
        Ok(_) => (),
        Err(_) => return Err(()),
    };
    Ok(())
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
