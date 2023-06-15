use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::io::{self, Read};
use std::path::{Path};
use chatgpt::client::ChatGPT;


#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    openai_api_key: String,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if Path::new(path).exists() {
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
        Ok(Config {
            openai_api_key: openai_api_key.trim().to_string(),
        })
    }

    pub fn create_chatgpt_client(&self) -> Result<ChatGPT, Box<dyn std::error::Error>> {
        let client: ChatGPT = ChatGPT::new(&self.openai_api_key)?;
        Ok(client)
    }
}