use core::fmt;
use std::{collections::HashMap};

use chatgpt::{prelude::ChatGPT, types::ChatMessage};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MyError {
    UUIDParseFail,
    FindByIDFail,
    EmitFail,
    ConversationWriteToDiskFail,
    NoConfigDirFail,
    UserNotLatestAuthorInConversationFail,
    ConversationEmptyFail,
    ConversationAIResponseFail,
}
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::UUIDParseFail => write!(f, "Failed to parse UUID"),
            MyError::FindByIDFail => write!(f, "Failed to find by ID"),
            MyError::EmitFail => write!(f, "Failed to emit"),
            MyError::ConversationWriteToDiskFail => write!(f, "Failed to write conversation to disk"),
            MyError::NoConfigDirFail => write!(f, "Failed identifying config directory"),
            MyError::UserNotLatestAuthorInConversationFail => write!(f, "User is not the latest author in the conversation"),
            MyError::ConversationEmptyFail => write!(f, "Conversation is empty"),
            MyError::ConversationAIResponseFail => write!(f, "Failed to get AI response"),
        }
    }
}
impl std::error::Error for MyError {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversationMessage {
    pub conversation_id: uuid::Uuid,
    pub id: uuid::Uuid,
    pub author: chatgpt::types::Role,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conversation {
    pub id: uuid::Uuid,
    pub title: String,
    pub history: Vec<ConversationMessage>,
}
impl Conversation {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            title: "Untitled Conversation".to_string(),
            history: Vec::new(),
        }
    }
    pub fn into_chatgpt_conversation(&self, chatgpt: ChatGPT) -> chatgpt::converse::Conversation {
        let history: Vec<chatgpt::types::ChatMessage> = self
            .history
            .iter()
            .map(|msg| ChatMessage {
                content: msg.content.clone(),
                role: msg.author,
            })
            .collect();
        chatgpt::converse::Conversation::new_with_history(chatgpt, history)
    }
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationTitleChangedEvent {
    pub conversation_id: uuid::Uuid,
    pub new_title: String,
}


pub struct ConversationManager {
    pub conversations: HashMap<Uuid, Conversation>,
}
impl ConversationManager {
    pub fn new() -> Self {
        Self {
            conversations: HashMap::new(),
        }
    }
    pub fn from_disk (path: &str) -> Result<Self, std::io::Error> {
        let file = std::fs::File::open(path)?;
        let conversations: HashMap<Uuid, Conversation> = serde_json::from_reader(file)?;
        Ok(Self {
            conversations
        })
    }
    pub fn write_to_disk(&self, path: &str) -> Result<(), std::io::Error> {
        let file = std::fs::File::create(path)?;
        serde_json::to_writer(file, &self.conversations)?;
        Ok(())
    }
}

