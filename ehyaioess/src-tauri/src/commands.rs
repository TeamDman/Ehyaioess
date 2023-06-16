// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use chatgpt::prelude::ChatGPT;
use std::collections::HashMap;
use tauri::{async_runtime::RwLock, Manager, State};

use crate::models::{
    Conversation, ConversationManager, ConversationMessage, ConversationTitleChangedEvent, MyError,
};

#[tauri::command(rename_all = "snake_case")]
pub async fn list_conversation_titles(
    conversation_manager: State<'_, RwLock<ConversationManager>>,
) -> Result<HashMap<String, String>, MyError> {
    let mgr = conversation_manager.read().await;
    let titlesById = mgr
        .conversations
        .iter()
        .map(|(id, conv)| (id.to_string(), conv.title.clone()))
        .collect();
    Ok(titlesById)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_conversation(
    conversation_manager: State<'_, RwLock<ConversationManager>>,
    conversation_id: &str,
) -> Result<Conversation, MyError> {
    let conversation_id =
        uuid::Uuid::parse_str(conversation_id).map_err(|_| MyError::FindByIDFail)?;
    let mgr = conversation_manager.read().await;
    let conversation = mgr
        .conversations
        .get(&conversation_id)
        .ok_or(MyError::FindByIDFail)?;
    Ok(conversation.clone())
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
    conversation_id: &str,
    new_title: &str,
) -> Result<(), MyError> {
    let conversation_id =
        uuid::Uuid::parse_str(conversation_id).map_err(|_| MyError::UUIDParseFail)?;
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
pub async fn new_user_message(
    app_handle: tauri::AppHandle,
    config: State<'_, crate::config::Config>,
    conversation_manager: State<'_, RwLock<ConversationManager>>,
    conversation_id: &str,
    content: &str,
) -> Result<ConversationMessage, MyError> {
    // get write lock
    let mut mgr = conversation_manager.write().await;
    // find the conversation by id
    let conv = mgr
        .conversations
        .get_mut(&uuid::Uuid::parse_str(conversation_id).unwrap())
        .ok_or(MyError::UUIDParseFail)?;
    // create the new message
    let msg = ConversationMessage {
        conversation_id: conv.id,
        id: uuid::Uuid::new_v4(),
        author: chatgpt::types::Role::User,
        content: content.to_string(),
    };
    // add the message to the conversation
    conv.history.push(msg.clone());
    // persist changes
    mgr.write_to_disk(&config.conversation_history_save_path)
        .map_err(|_| MyError::ConversationWriteToDiskFail)?;
    // release lock
    drop(mgr);
    // notify listeners
    app_handle
        .emit_all("conversation_message_added", msg.clone())
        .map_err(|_| MyError::EmitFail)?;
    Ok(msg.clone())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn generate_assistant_message(
    app_handle: tauri::AppHandle,
    config: State<'_, crate::config::Config>,
    chatgpt: State<'_, ChatGPT>,
    conversation_manager: State<'_, RwLock<ConversationManager>>,
    conversation_id: &str,
) -> Result<ConversationMessage, MyError> {
    // get write lock
    let mut mgr = conversation_manager.write().await;
    // find the conversation by id
    let conv = mgr
        .conversations
        .get_mut(&uuid::Uuid::parse_str(conversation_id).unwrap())
        .ok_or(MyError::UUIDParseFail)?;

    let latest_message = conv.history.last().ok_or(MyError::ConversationEmptyFail)?;
    if latest_message.author != chatgpt::types::Role::User {
        return Err(MyError::UserNotLatestAuthorInConversationFail);
    }

    let mut ai_conversation = conv.into_chatgpt_conversation(chatgpt.inner().clone());
    // remove the last message from the conversation
    let ai_prompt = ai_conversation
        .history
        .pop()
        .ok_or(MyError::ConversationEmptyFail)?;
    let ai_response = ai_conversation
        .send_message(ai_prompt.content)
        .await
        .map_err(|_| MyError::ConversationAIResponseFail)?;

    // create the new message
    let msg = ConversationMessage {
        conversation_id: conv.id,
        id: uuid::Uuid::new_v4(),
        author: chatgpt::types::Role::Assistant,
        content: ai_response.message().content.clone(),
    };

    // add the message to the conversation
    conv.history.push(msg.clone());
    // persist changes
    mgr.write_to_disk(&config.conversation_history_save_path)
        .map_err(|_| MyError::ConversationWriteToDiskFail)?;
    // release lock
    drop(mgr);
    // notify listeners
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
