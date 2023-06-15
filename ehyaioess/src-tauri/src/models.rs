use std::{collections::HashMap};

use chatgpt::{prelude::ChatGPT, types::ChatMessage};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversationMessage {
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
    pub fn into_chatgpt_conversation(&self, client: ChatGPT) -> chatgpt::converse::Conversation {
        let history: Vec<chatgpt::types::ChatMessage> = self
            .history
            .iter()
            .map(|msg| ChatMessage {
                content: msg.content.clone(),
                role: msg.author,
            })
            .collect();
        chatgpt::converse::Conversation::new_with_history(client, history)
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
    pub fn new_conversation(&mut self) -> &Conversation {
        let conv = Conversation::new();
        let id = conv.id;
        self.conversations.insert(id, conv);
        self.conversations.get(&id).unwrap()
    }
}

