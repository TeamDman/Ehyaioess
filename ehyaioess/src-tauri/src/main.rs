// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
use config::Config;
use std::sync::Arc;
use chatgpt::client::ChatGPT;

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
}



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
async fn greet(name: &str) -> Result<String,String> {
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
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}