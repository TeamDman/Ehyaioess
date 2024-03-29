use core::fmt;
use std::{
    any::TypeId,
    collections::HashMap, borrow::Cow,
};

use chatgpt::{prelude::ChatGPT, types::ChatMessage};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::Error;
use uuid::Uuid;
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub enum MyError {
    UUIDParseFail,
    FindByIDFail,
    EmitFail,
    ConversationWriteToDiskFail,
    NoConfigDirFail,
    UserNotLatestAuthorInConversationFail,
    ConversationEmptyFail,
    ConversationAIResponseFail,
    DirListFail,
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
            MyError::DirListFail => write!(f, "Failed to list directory"),
        }
    }
}
impl std::error::Error for MyError {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Type)]
pub enum ChatRole {
    System,
    Assistant,
    User
}

impl From<chatgpt::types::Role> for ChatRole {
    fn from(role: chatgpt::types::Role) -> Self {
        match role {
            chatgpt::types::Role::System => ChatRole::System,
            chatgpt::types::Role::Assistant => ChatRole::Assistant,
            chatgpt::types::Role::User => ChatRole::User,
        }
    }
}

impl From<ChatRole> for chatgpt::types::Role {
    fn from(chat_role: ChatRole) -> Self {
        match chat_role {
            ChatRole::System => chatgpt::types::Role::System,
            ChatRole::Assistant => chatgpt::types::Role::Assistant,
            ChatRole::User => chatgpt::types::Role::User,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub struct ConversationMessageAddedEvent {
    pub author: ChatRole,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub struct ConversationTitleChangedEvent {
    pub new_title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub enum ConversationEvent {
    MessageAdded(ConversationMessageAddedEvent),
    TitleChange(ConversationTitleChangedEvent),
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




fn serialize_timestamp<S>(timestamp: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let timestamp_str = timestamp.to_string();
    serializer.serialize_str(&timestamp_str)
}

fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<i64>().map_err(Error::custom)
}
#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub struct ConversationEventRecord {
    pub id: uuid::Uuid,
    pub conversation_id: Uuid,
    #[serde(serialize_with = "serialize_timestamp", deserialize_with = "deserialize_timestamp")]
    pub timestamp: i64,
    pub event: ConversationEvent,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub struct Conversation {
    pub id: uuid::Uuid,
    pub history: Vec<ConversationEventRecord>,
}

const DEFAULT_CONVERSATION_TITLE: &str = "Untitled Conversation";
impl Conversation {
    pub fn new() -> Self {
        let conv = Self {
            id: uuid::Uuid::new_v4(),
            history: Vec::new(),
        };
        conv
    }
    pub fn get_latest_event<T: 'static>(&self) -> Option<&ConversationEventRecord> {
        self.history
            .iter()
            // .filter(|record| Any::type_id(&record.event as &dyn Any) == TypeId::of::<T>())
            .filter(|record| match record.event {
                ConversationEvent::TitleChange(_) => TypeId::of::<T>() == TypeId::of::<ConversationTitleChangedEvent>(),
                ConversationEvent::MessageAdded(_) => TypeId::of::<T>() == TypeId::of::<ConversationMessageAddedEvent>(),
            })
            .max_by_key(|record| record.timestamp)
    }
    pub fn add_event<E: Into<ConversationEvent>>(&mut self, event: E) -> &ConversationEventRecord {
        let record = ConversationEventRecord {
            id: uuid::Uuid::new_v4(),
            conversation_id: self.id,
            timestamp: chrono::Utc::now().timestamp_millis(),
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
                        role: msg.author.into(),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_title() {
        let mut conv = Conversation::new();
        assert_eq!(
            conv.get_title().as_ref(),
            DEFAULT_CONVERSATION_TITLE
        );
        let latest = conv.add_event(ConversationTitleChangedEvent {
            new_title: "New Title".to_string(),
        }).id;
        assert_eq!(conv.get_latest_event::<ConversationTitleChangedEvent>().unwrap().id, latest);
        assert_eq!(conv.get_title().as_ref(), "New Title");
        conv.add_event(ConversationTitleChangedEvent {
            new_title: "Newer Title".to_string(),
        });
        assert_eq!(conv.get_title().as_ref(), "Newer Title");
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
