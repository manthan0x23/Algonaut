use core::fmt;
use std::fmt::Display;

use chrono::{DateTime, Utc};
use common::{
    id::ShortId,
    types::session::{UserMinimal, UserRoomType},
};
use database::entity;
use serde::{Deserialize, Serialize};

/// 16 zeros string with a hash `#0000000000000000`
const SYSTEM_ID: &str = "#00000000000000000";

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutgoingMessageIden {
    Broadcast,
    Chat,
    Execution,
    RoomMembers,
    CrdtUpdate,
    Error,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OutgoingMessage {
    Broadcast(Broadcast),
    Chat(OutgoingChat),
    RoomMembers(RoomMembers),
    Error(OutgoingError),
}

// ----------------- MEMBERS -----------------

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoomMember {
    pub uid: ShortId,
    pub name: Option<String>,
    pub email: String,
    pub role: UserRoomType,
}

#[derive(Serialize, Deserialize)]
pub struct RoomMembers {
    iden: OutgoingMessageIden,
    members: Vec<RoomMember>,
}

impl RoomMembers {
    pub fn announce(members: Vec<RoomMember>) -> Self {
        Self {
            iden: OutgoingMessageIden::RoomMembers,
            members,
        }
    }
}

// ----------------- BROADCAST -----------------

#[derive(Serialize, Deserialize)]
pub struct Broadcast {
    iden: OutgoingMessageIden,
    message: String,
}

impl Broadcast {
    pub fn announce(message: String) -> Self {
        Self {
            iden: OutgoingMessageIden::Broadcast,
            message,
        }
    }
}

// ----------------- CHAT -----------------

#[derive(Serialize, Deserialize)]
pub struct OutgoingChat {
    pub iden: OutgoingMessageIden,
    pub r#type: ChatType,

    #[serde(flatten)]
    pub chat: entity::chat::Model,

    pub sender: UserMinimal,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatType {
    Announcement,
    Message,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChatMessageType {
    Text,
    Video,
    Audio,
    File,
    Image,
}

impl Display for ChatMessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str = match self {
            ChatMessageType::Text => "text",
            ChatMessageType::Video => "video",
            ChatMessageType::Audio => "audio",
            ChatMessageType::File => "file",
            ChatMessageType::Image => "image",
        };
        write!(f, "{}", as_str)
    }
}

impl OutgoingChat {
    pub fn new(chat: entity::chat::Model, sender: UserMinimal, chat_type: ChatType) -> Self {
        Self {
            iden: OutgoingMessageIden::Chat,
            r#type: chat_type,
            chat,
            sender,
        }
    }
}

// ----------------- ERROR -----------------

#[derive(Serialize, Deserialize)]
pub struct OutgoingError {
    pub iden: OutgoingMessageIden,
    pub message: String,
    pub errors: Vec<String>,
}

impl OutgoingError {
    pub fn new(message: impl Into<String>, errors: Vec<String>) -> Self {
        Self {
            iden: OutgoingMessageIden::Error,
            message: message.into(),
            errors,
        }
    }
}

// ----------------- CRDT -----------------

#[derive(Serialize, Deserialize)]
pub struct OutgoingCrdtUpdate {
    pub iden: OutgoingMessageIden,
    pub update: Vec<Vec<u8>>,
}

impl OutgoingCrdtUpdate {
    pub fn new(update: Vec<Vec<u8>>) -> Self {
        Self {
            iden: OutgoingMessageIden::CrdtUpdate,
            update,
        }
    }
}
