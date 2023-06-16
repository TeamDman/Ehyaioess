use serde::{Deserialize, Serialize};
use tauri::api::path::config_dir;
use std::fs::File;
use std::io::Write;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use chatgpt::client::ChatGPT;

use crate::models::MyError;


#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    openai_api_key: String,
    pub conversation_history_save_path: String,
}

impl Config {
    pub fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        if let Some(mut path) = config_dir() {
            path.push("ehyaioess");
            if !Path::new(&path).exists() {
                std::fs::create_dir_all(&path)?;
            }
            path.push("config.json");
            return Ok(path);
        } else {
            return Err(Box::new(MyError::NoConfigDirFail));
        }
    }
    pub fn from_disk() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Config::get_config_path()?;
        println!("Config path: {:?}", path);
        if path.exists() {
            let mut file = File::open(path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let config: Config = serde_json::from_str(&contents)?;
            Ok(config)
        } else {
            let config = Config::from_user()?;
            let json = serde_json::to_string_pretty(&config)?;
            let mut file = File::create(path)?;
            file.write_all(json.as_bytes())?;
            Ok(config)
        }
    }

    fn from_user() -> Result<Self, Box<dyn std::error::Error>> {
        println!("Please enter your OpenAI API Key: ");
        let mut openai_api_key = String::new();
        io::stdin().read_line(&mut openai_api_key)?;

        let mut conversation_history_save_path = match config_dir() {
            Some(mut path) => {
                path.push("ehyaioess");
                path.push("conversations.json");
                path.to_str().unwrap().to_string()
            }
            None => {
                String::new()
            }
        };
        println!("Please enter the path to save conversation history to (leave blank for default): ");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;
        if user_input.trim().len() > 0 {
            conversation_history_save_path = user_input.trim().to_string();
        }

        Ok(Config {
            openai_api_key: openai_api_key.trim().to_string(),
            conversation_history_save_path,
        })
    }

    pub fn create_chatgpt_client(&self) -> Result<ChatGPT, Box<dyn std::error::Error>> {
        let client: ChatGPT = ChatGPT::new(&self.openai_api_key)?;
        Ok(client)
    }
}