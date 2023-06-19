use core::fmt;
use std::{
    any::{Any, TypeId},
    collections::HashMap, borrow::Cow,
};

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
            MyError::ConversationWriteToDiskFail => {
                write!(f, "Failed to write conversation to disk")
            }
            MyError::NoConfigDirFail => write!(f, "Failed identifying config directory"),
            MyError::UserNotLatestAuthorInConversationFail => {
                write!(f, "User is not the latest author in the conversation")
            }
            MyError::ConversationEmptyFail => write!(f, "Conversation is empty"),
            MyError::ConversationAIResponseFail => write!(f, "Failed to get AI response"),
        }
    }
}
impl std::error::Error for MyError {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversationMessageAddedEvent {
    pub author: chatgpt::types::Role,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversationTitleChangedEvent {
    pub new_title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversationCreatedEvent {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConversationEvent {
    MessageAdded(ConversationMessageAddedEvent),
    TitleChange(ConversationTitleChangedEvent),
    Created(ConversationCreatedEvent),
}
impl From<ConversationMessageAddedEvent> for ConversationEvent {
    fn from(event: ConversationMessageAddedEvent) -> Self {
        ConversationEvent::MessageAdded(event)
    }
}

impl From<ConversationTitleChangedEvent> for ConversationEvent {
    fn from(event: ConversationTitleChangedEvent) -> Self {
        ConversationEvent::TitleChange(event)
    }
}

impl From<ConversationCreatedEvent> for ConversationEvent {
    fn from(_: ConversationCreatedEvent) -> Self {
        ConversationEvent::Created(ConversationCreatedEvent {})
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversationEventRecord {
    pub id: uuid::Uuid,
    pub conversation_id: Uuid,
    pub timestamp: i64,
    pub event: ConversationEvent,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conversation {
    pub id: uuid::Uuid,
    pub history: Vec<ConversationEventRecord>,
}

const DEFAULT_CONVERSATION_TITLE: &str = "Untitled Conversation";
impl Conversation {
    pub fn new() -> Self {
        let mut conv = Self {
            id: uuid::Uuid::new_v4(),
            history: Vec::new(),
        };
        conv.add_event(ConversationCreatedEvent {});
        conv
    }
    pub fn get_latest_event<T: 'static>(&self) -> Option<&ConversationEventRecord> {
        self.history
            .iter()
            .filter(|record| Any::type_id(&record.event as &dyn Any) == TypeId::of::<T>())
            .max_by_key(|record| record.timestamp)
    }
    pub fn add_event<E: Into<ConversationEvent>>(&mut self, event: E) -> &ConversationEventRecord {
        let record = ConversationEventRecord {
            id: uuid::Uuid::new_v4(),
            conversation_id: self.id,
            timestamp: chrono::Utc::now().timestamp(),
            event: event.into(),
        };
        self.history.push(record);
        self.history.last().unwrap()
    }
    pub fn into_chatgpt_conversation(&self, chatgpt: ChatGPT) -> chatgpt::converse::Conversation {
        let history: Vec<chatgpt::types::ChatMessage> = self
            .history
            .iter()
            .filter_map(|record| {
                if let ConversationEvent::MessageAdded(msg) = &record.event {
                    Some(ChatMessage {
                        content: msg.content.clone(),
                        role: msg.author,
                    })
                } else {
                    None
                }
            })
            .collect();
        chatgpt::converse::Conversation::new_with_history(chatgpt, history)
    }
    pub fn get_title(&self) -> Cow<'_, String> {
        self.get_latest_event::<ConversationTitleChangedEvent>()
            .and_then(|record| {
                if let ConversationEvent::TitleChange(event) = &record.event {
                    Some(Cow::Borrowed(&event.new_title))
                } else {
                    None
                }
            })
            .unwrap_or_else(|| Cow::Owned(DEFAULT_CONVERSATION_TITLE.to_string()))
    }
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
    pub fn from_disk(path: &str) -> Result<Self, std::io::Error> {
        let file = std::fs::File::open(path)?;
        let conversations: HashMap<Uuid, Conversation> = serde_json::from_reader(file)?;
        Ok(Self { conversations })
    }
    pub fn write_to_disk(&self, path: &str) -> Result<(), std::io::Error> {
        let file = std::fs::File::create(path)?;
        serde_json::to_writer(file, &self.conversations)?;
        Ok(())
    }
}
