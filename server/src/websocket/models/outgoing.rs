use chrono::{DateTime, Utc};
use common::id::ShortId;
use serde::{Deserialize, Serialize};

/// 16 zeros string with a hash `#0000000000000000`
const SYSTEM_ID: &str = "#00000000000000000";

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutgoingMessageIden {
    Broadcast,
    Chat,
    Execution,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OutgoingMessage {
    Broadcast(Broadcast),
    Chat(Chat),
}

#[derive(Serialize, Deserialize)]
pub struct Broadcast {
    pub iden: OutgoingMessageIden,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct Chat {
    pub iden: OutgoingMessageIden,
    pub r#type: ChatType,
    pub message: Option<String>,
    pub from: ShortId,
    pub message_type: ChatMessageType,
    pub url: Option<String>,
    pub text: Option<String>,
    pub blob: Option<Vec<u8>>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatType {
    Announcement,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatMessageType {
    Text,
    Video,
    Audio,
    File,
    Image,
}
