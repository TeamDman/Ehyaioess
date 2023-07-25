// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use chatgpt::prelude::ChatGPT;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::{async_runtime::RwLock, Manager, State};

use crate::{
    models::{
        Conversation, ConversationEvent, ConversationManager, ConversationMessageAddedEvent,
        ConversationTitleChangedEvent, MyError,
    },
    events::{
        ConversationMessageAddedEventPayload, ConversationMessagePayload,
        ConversationTitleChangedEventPayload,
    },
};

#[tauri::command]
#[specta::specta]
pub async fn list_conversation_titles(
    conversation_manager: State<'_, RwLock<ConversationManager>>,
) -> Result<HashMap<String, String>, MyError> {
    let mgr = conversation_manager.read().await;
    let titles_by_id = mgr
        .conversations
        .iter()
        .map(|(id, conv)| (id.to_string(), conv.get_title().into_owned()))
        .collect();
    Ok(titles_by_id)
}

#[tauri::command]
#[specta::specta]
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

#[tauri::command]
#[specta::specta]
pub async fn get_conversation_title(
    conversation_manager: State<'_, RwLock<ConversationManager>>,
    conversation_id: &str,
) -> Result<String, MyError> {
    let conversation_id =
        uuid::Uuid::parse_str(conversation_id).map_err(|_| MyError::FindByIDFail)?;
    let mgr = conversation_manager.read().await;
    let conversation = mgr
        .conversations
        .get(&conversation_id)
        .ok_or(MyError::FindByIDFail)?;
    Ok(conversation.get_title().into_owned())
}

#[tauri::command]
#[specta::specta]
pub async fn get_conversation_messages(
    conversation_manager: State<'_, RwLock<ConversationManager>>,
    conversation_id: &str,
) -> Result<Vec<ConversationMessagePayload>, MyError> {
    let conversation_id =
        uuid::Uuid::parse_str(conversation_id).map_err(|_| MyError::FindByIDFail)?;
    let mgr = conversation_manager.read().await;
    let conversation = mgr
        .conversations
        .get(&conversation_id)
        .ok_or(MyError::FindByIDFail)?;
    let message_events = conversation
        .history
        .iter()
        .filter_map(|record| {
            if let ConversationEvent::MessageAdded(msg) = &record.event {
                Some(ConversationMessagePayload {
                    author: msg.author,
                    content: msg.content.clone(),
                })
            } else {
                None
            }
        })
        .collect();
    Ok(message_events)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationAddedEvent {
    pub conversation_id: uuid::Uuid,
    pub title: String,
}
#[tauri::command]
#[specta::specta]
pub async fn new_conversation(
    conversation_manager: State<'_, RwLock<ConversationManager>>,
    config: State<'_, crate::config::Config>,
    app_handle: tauri::AppHandle,
) -> Result<Conversation, MyError> {
    let mut mgr = conversation_manager.write().await;
    let conv = Conversation::new();

    mgr.conversations.insert(conv.id, conv.clone());
    mgr.write_to_disk(&config.conversation_history_save_path)
        .map_err(|_| MyError::ConversationWriteToDiskFail)?;

    // Drop the lock before emitting events.
    drop(mgr);

    app_handle
        .emit_all(
            "new_conversation",
            ConversationAddedEvent {
                conversation_id: conv.id,
                title: conv.get_title().into_owned(),
            },
        )
        .map_err(|_| MyError::EmitFail)?;
    Ok(conv)
}

#[tauri::command]
#[specta::specta]
pub async fn set_conversation_title(
    conversation_manager: State<'_, RwLock<ConversationManager>>,
    config: State<'_, crate::config::Config>,
    app_handle: tauri::AppHandle,
    conversation_id: &str,
    new_title: &str,
) -> Result<(), MyError> {
    let conversation_id =
        uuid::Uuid::parse_str(conversation_id).map_err(|_| MyError::UUIDParseFail)?;
    let new_title_trimmed = new_title.trim();

    {
        let mut mgr = conversation_manager.write().await;
        let conv = mgr
            .conversations
            .get_mut(&conversation_id)
            .ok_or(MyError::FindByIDFail)?;
        let current_title = conv.get_title();
        if current_title.as_ref() == new_title_trimmed {
            return Ok(());
        }
        conv.add_event(ConversationTitleChangedEvent {
            new_title: new_title_trimmed.to_string(),
        })
    };

    conversation_manager
        .read()
        .await
        .write_to_disk(&config.conversation_history_save_path)
        .map_err(|_| MyError::ConversationWriteToDiskFail)?;

    app_handle
        .emit_all(
            "conversation_title_changed",
            ConversationTitleChangedEventPayload {
                conversation_id,
                new_title: new_title_trimmed.to_string(),
            },
        )
        .map_err(|_| MyError::EmitFail)?;

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn new_conversation_user_message(
    app_handle: tauri::AppHandle,
    config: State<'_, crate::config::Config>,
    conversation_manager: State<'_, RwLock<ConversationManager>>,
    conversation_id: &str,
    content: &str,
) -> Result<(), MyError> {
    let conversation_id =
        uuid::Uuid::parse_str(conversation_id).map_err(|_| MyError::UUIDParseFail)?;

    {
        let mut mgr = conversation_manager.write().await;
        let conv = mgr
            .conversations
            .get_mut(&conversation_id)
            .ok_or(MyError::UUIDParseFail)?;
        conv.add_event(ConversationMessageAddedEvent {
            author: crate::models::ChatRole::User,
            content: content.to_string(),
        })
        .clone()
    };

    conversation_manager
        .read()
        .await
        .write_to_disk(&config.conversation_history_save_path)
        .map_err(|_| MyError::ConversationWriteToDiskFail)?;

    app_handle
        .emit_all(
            "conversation_message_added",
            ConversationMessageAddedEventPayload {
                conversation_id,
                author: crate::models::ChatRole::User,
                content: content.to_string(),
            },
        )
        .map_err(|_| MyError::EmitFail)?;

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn new_conversation_assistant_message(
    app_handle: tauri::AppHandle,
    config: State<'_, crate::config::Config>,
    chatgpt: State<'_, ChatGPT>,
    conversation_manager: State<'_, RwLock<ConversationManager>>,
    conversation_id: &str,
) -> Result<(), MyError> {
    let conversation_id =
        uuid::Uuid::parse_str(conversation_id).map_err(|_| MyError::UUIDParseFail)?;

    let response = {
        let mut mgr = conversation_manager.write().await;
        let conv = mgr
            .conversations
            .get_mut(&conversation_id)
            .ok_or(MyError::UUIDParseFail)?;

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

        let response = ai_response.message().content.clone();
        conv.add_event(ConversationMessageAddedEvent {
            author: crate::models::ChatRole::Assistant,
            content: response.clone(),
        });
        response
    };

    conversation_manager
        .read()
        .await
        .write_to_disk(&config.conversation_history_save_path)
        .map_err(|_| MyError::ConversationWriteToDiskFail)?;

    app_handle
        .emit_all(
            "conversation_message_added",
            ConversationMessageAddedEventPayload {
                conversation_id,
                author: crate::models::ChatRole::Assistant,
                content: response,
            },
        )
        .map_err(|_| MyError::EmitFail)?;

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn list_files() -> Result<Vec<String>, MyError> {
    let res = std::fs::read_dir("./")
        .map_err(|_| MyError::DirListFail)?
        .map(|res| res.map(|e| e.path().display().to_string()))
        .collect::<Result<Vec<String>, std::io::Error>>()
        .map_err(|_| MyError::DirListFail)?;

    Ok(res)
}



#[cfg(test)]
mod test {
    use crate::{commands, events};

    #[test]
    fn build_command_type_definitions() {
        tauri_specta::ts::export_with_cfg(
            specta::collect_types![
                commands::list_conversation_titles,
                commands::get_conversation_messages,
                commands::get_conversation_title,
                commands::get_conversation,
                commands::new_conversation,
                commands::set_conversation_title,
                commands::new_conversation_user_message,
                commands::new_conversation_assistant_message,
                commands::list_files,
                events::wrap_event_payloads,
            ]
            .unwrap(),
            specta::ts::ExportConfiguration::new().bigint(specta::ts::BigIntExportBehavior::String),
            "../src/lib/bindings.ts",
        )
        .unwrap();
    }
}