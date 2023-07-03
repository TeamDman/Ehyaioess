// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use chatgpt::prelude::ChatGPT;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::{async_runtime::RwLock, Manager, State};

use crate::{
    models::{
        Conversation, ConversationEvent, ConversationManager,
        ConversationMessageAddedEvent, ConversationTitleChangedEvent, MyError,
    },
    payloads::{
        ConversationMessageAddedEventPayload, ConversationMessagePayload,
        ConversationTitleChangedEventPayload,
    },
};

#[cfg(test)]
mod test {
    fn rust_type_to_ts(rust_type: &syn::Type) -> String {
        match rust_type {
            syn::Type::Path(type_path) if type_path.qself.is_none() => {
                let ident = &type_path.path.segments.last().unwrap().ident;
                match ident.to_string().as_str() {
                    "str" => "string".to_owned(),
                    "()" => "void".to_owned(),
                    "Result" => {
                        match &type_path.path.segments.last().unwrap().arguments {
                            syn::PathArguments::AngleBracketed(angle_bracketed_data) => {
                                if let Some(syn::GenericArgument::Type(ty)) = angle_bracketed_data.args.first() {
                                    let inner_ts = rust_type_to_ts(ty);
                                    if inner_ts == "void" {
                                        "Promise<void>".to_owned()
                                    } else {
                                        format!("Promise<{}>", inner_ts)
                                    }
                                } else {
                                    panic!("Result without inner type")
                                }
                            },
                            _ => panic!("Unsupported angle type: {}", ident.to_string()),
                        }
                    },
                    "Vec" => {
                        match &type_path.path.segments.last().unwrap().arguments {
                            syn::PathArguments::AngleBracketed(angle_bracketed_data) => {
                                if let Some(syn::GenericArgument::Type(ty)) = angle_bracketed_data.args.first() {
                                    format!("Array<{}>", rust_type_to_ts(ty))
                                } else {
                                    panic!("Vec without inner type")
                                }
                            },
                            _ => panic!("Unsupported angle type: {}", ident.to_string()),
                        }
                    },
                    _ => ident.to_string(),
                }
            },
            syn::Type::Reference(type_reference) => {
                if let syn::Type::Path(type_path) = *type_reference.elem.clone() {
                    let ident = &type_path.path.segments.last().unwrap().ident;
                    match ident.to_string().as_str() {
                        "str" => "string".to_owned(),
                        _ => panic!("Unsupported type: &{}", ident.to_string()),
                    }
                } else {
                    panic!("Unsupported ref type: {}", quote::quote! {#type_reference}.to_string())
                }
            },
            syn::Type::Tuple(tuple_type) if tuple_type.elems.is_empty() => {
                "void".to_owned()
            },
            _ => panic!("Unsupported type: {}", quote::quote! {#rust_type}.to_string()),
        }
    }
    


    #[test]
    fn list_commands() {
        let contents = std::fs::read_to_string("src/commands.rs").unwrap();
        let ast = syn::parse_file(&contents).unwrap();
        ast.items.iter().for_each(|item| {
            if let syn::Item::Fn(item_fn) = item {
                let tauri_command_attr = item_fn.attrs.iter()
                    .find(|attr| {
                        attr.path().segments.iter().map(|seg| seg.ident.to_string()).collect::<Vec<_>>() == ["tauri", "command"]
                    });

                if tauri_command_attr.is_some() {
                    println!("{}", item_fn.sig.ident);
                    // print the typescript definition
                    let mut ts_def = format!("function {}(", item_fn.sig.ident);
                    let mut ts_args = vec![];
                    item_fn.sig.inputs.iter().for_each(|arg| {
                        if let syn::FnArg::Typed(pat_type) = arg {
                            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                                // Filter out State and AppHandle parameters
                                let ty_string = quote::quote! {#pat_type.ty}.to_string();
                                if !ty_string.contains("State") && !ty_string.contains("AppHandle") {
                                    let ts_type = rust_type_to_ts(&pat_type.ty);
                                    ts_args.push(format!("\n\t{}: {}", pat_ident.ident, ts_type));
                                    // ts_args.push(format!("\n\t{}: {}", pat_ident.ident, ty_string));
                                }
                            }
                        }
                    });
                    ts_def.push_str(&ts_args.join(","));
                    ts_def.push_str("\n): Promise<");
                    if let syn::ReturnType::Type(_, ty) = &item_fn.sig.output {
                        let ts_type = rust_type_to_ts(ty);
                        ts_def.push_str(&ts_type);
                        // ts_def.push_str(&quote::quote! {#ty}.to_string());
                    }
                    ts_def.push_str(">;");
                    println!("{}", ts_def);

                    println!("\n\n")
                }
            }
        });
    }
}

#[tauri::command(rename_all = "snake_case")]
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

#[tauri::command(rename_all = "snake_case")]
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
#[tauri::command(rename_all = "snake_case")]
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

#[tauri::command(rename_all = "snake_case")]
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
            author: chatgpt::types::Role::User,
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
                author: chatgpt::types::Role::User,
                content: content.to_string(),
            },
        )
        .map_err(|_| MyError::EmitFail)?;

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
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
            author: chatgpt::types::Role::Assistant,
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
                author: chatgpt::types::Role::Assistant,
                content: response,
            },
        )
        .map_err(|_| MyError::EmitFail)?;

    Ok(())
}



#[tauri::command(rename_all = "snake_case")]
pub async fn list_files() -> Result<Vec<String>, MyError> {
    let res = std::fs::read_dir("./").map_err(|_| MyError::DirListFail)?
    .map(|res| res.map(|e| e.path().display().to_string()))
    .collect::<Result<Vec<String>, std::io::Error>>().map_err(|_| MyError::DirListFail)?;
    
    Ok(res)
}