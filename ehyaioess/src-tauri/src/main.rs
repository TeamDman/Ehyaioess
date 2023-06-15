// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
use chatgpt::{client::ChatGPT, types::ChatMessage};
use config::Config;
use serde::{Serialize, Deserialize};
use std::{collections::HashMap, sync::{Arc, RwLock}};
mod stuff;

// Globally accessible state
lazy_static::lazy_static! {

    static ref CONFIG: Arc<Config> = Arc::new(
        match Config::from_file("ehyaioess.conf.secret.json") {
            Ok(conf) => conf,
            Err(e) => {
                eprintln!("Failed to load configuration: {}", e);
                std::process::exit(1);
            }
        }
    );
    static ref STATE: RwLock<stuff::State> = 
        RwLock::new(stuff::State::new(
            Arc::clone(&CONFIG),
            match CONFIG.create_chatgpt_client() {
                Ok(client) => client,
                Err(e) => {
                    eprintln!("Failed to create ChatGPT client: {}", e);
                    std::process::exit(1);
                }
            }
        )
    );
}



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[derive(Debug, Serialize, Deserialize)]
struct ConversationModel {
    id: uuid::Uuid,
    title: String,
    history: Vec<ChatMessage>,
}
#[tauri::command(rename_all = "snake_case")]
async fn list_conversations() -> Result<HashMap<uuid::Uuid, ConversationModel>, String> {
    println!("list_conversations");
    let state = STATE.read().unwrap();
    Ok(state
        .conversations
        .iter()
        .map(|(id, conv)| {
            (
                id.clone(),
                ConversationModel {
                    id: conv.id.clone(),
                    title: conv.title.clone(),
                    history: conv.conversation.history.clone(),
                },
            )
        })
        .collect())
}

#[tauri::command(rename_all = "snake_case")]
fn new_conversation() -> Result<ConversationModel, ()> {
    println!("new_conversation");
    let mut state = STATE.write().unwrap();
    let conv = state.new_conversation();
    let model = ConversationModel {
        id: conv.id.clone(),
        title: conv.title.clone(),
        history: conv.conversation.history.clone(),
    };
    Ok(model)
}

#[tauri::command(rename_all="snake_case")]
fn rename_conversation(app_handle: tauri::AppHandle, id: &str, new_title: &str) -> Result<(),()> {
    println!("rename_conversation");
    let mut state = STATE.write().unwrap();
    let id = match uuid::Uuid::parse_str(id) {
        Ok(id) => id,
        Err(_) => return Err(()),
    };
    let conv = match state.conversations.get_mut(&id) {
        Some(conv) => conv,
        None => return Err(()),
    };
    conv.title = new_title.to_string();
    Ok(())
}

#[tauri::command]
async fn greet(name: &str) -> Result<String, String> {
    // Clone the Arc to get a new reference to the config
    let config = Arc::clone(&CONFIG);

    let client: ChatGPT = match config.create_chatgpt_client() {
        Ok(client) => client,
        Err(e) => return Err(e.to_string()),
    };

    let prompt = format!("Hello from {}!", name);
    let response_result = client.send_message(prompt).await;
    let response = match response_result {
        Ok(response) => response.message().content.clone(),
        Err(e) => return Err(e.to_string()), // if there's an error sending the message, return it
    };

    Ok(response) // if everything is okay, return the content
}

fn main() {
    // println!("{:#?}", *CONFIG);
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![greet, list_conversations, new_conversation])
        .setup(|app| {
            std::thread::spawn(move || {

            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
