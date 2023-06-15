use std::{sync::{Arc, mpsc}, collections::HashMap};

use chatgpt::prelude::ChatGPT;
use uuid::Uuid;

use crate::config::Config;


pub struct Conversation {
    pub id: Uuid,
    pub title: String,
    pub conversation: chatgpt::converse::Conversation,
}

pub struct State {
    config: Arc<Config>,
    chatgpt: ChatGPT,
    pub conversations: HashMap<Uuid, Conversation>,
}

impl State {
    // Add methods here
    pub fn new(config: Arc<Config>, client: ChatGPT) -> Self {
        Self{
            config: config,
            chatgpt: client,
            conversations: HashMap::new(),
        }
    }

    pub fn new_conversation(&mut self) -> &Conversation {
        let conv = Conversation {
            id: uuid::Uuid::new_v4(),
            title: "New Conversation".to_string(),
            conversation: self.chatgpt.new_conversation(),
        };
        let id = conv.id;
        self.conversations.insert(id, conv);
        self.conversations.get(&id).unwrap()
    }
}
