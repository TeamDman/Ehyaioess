// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
use config::Config;
use models::ConversationManager;
use std::time::{Duration, Instant};
use tauri::{async_runtime::RwLock, Manager};
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

mod commands;
mod models;
mod payloads;

fn main() {
    let config = match Config::from_disk() {
        Ok(conf) => conf,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };
    let chatgpt = match config.create_chatgpt_client() {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to create ChatGPT client: {}", e);
            std::process::exit(1);
        }
    };
    let conversation_manager =
        ConversationManager::from_disk(&config.conversation_history_save_path)
            .unwrap_or_else(|_| ConversationManager::new());

    tauri::Builder::default()
        .manage(config)
        .manage(chatgpt)
        .manage(RwLock::new(conversation_manager))
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            commands::list_conversation_titles,
            commands::get_conversation_messages,
            commands::get_conversation_title,
            commands::get_conversation,
            commands::new_conversation,
            commands::set_conversation_title,
            commands::new_conversation_user_message,
            commands::new_conversation_assistant_message,
        ])
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            {
                // save window state on move
                let last_save_time = std::cell::Cell::new(Instant::now() - Duration::from_secs(1));
                let app_handle = app.app_handle();
                window.on_window_event(move |e| {
                    match e {
                        tauri::WindowEvent::Moved(_) => {
                            let now = Instant::now();
                            // Only call save_window_state if a second or more has passed.
                            if now - last_save_time.get() >= Duration::from_secs(1) {
                                app_handle.save_window_state(StateFlags::all()).unwrap();
                                last_save_time.set(now);
                                println!("Saved window state")
                            }
                        }
                        _ => {}
                    }
                });
            }
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
