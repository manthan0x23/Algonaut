use chrono::{DateTime, Utc};
use common::{id::ShortId, types::session::UserRoomType};
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
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OutgoingMessage {
    Broadcast(Broadcast),
    Chat(Chat),
    RoomMembers(RoomMembers),
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
pub struct Chat {
    iden: OutgoingMessageIden,
    r#type: ChatType,
    message: Option<String>,
    from: ShortId,
    message_type: ChatMessageType,
    url: Option<String>,
    text: Option<String>,
    blob: Option<Vec<u8>>,
    timestamp: DateTime<Utc>,
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

impl Chat {
    fn new(
        r#type: ChatType,
        message: Option<String>,
        from: ShortId,
        message_type: ChatMessageType,
        url: Option<String>,
        text: Option<String>,
        blob: Option<Vec<u8>>,
        timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            iden: OutgoingMessageIden::Chat,
            r#type,
            message,
            from,
            message_type,
            url,
            text,
            blob,
            timestamp,
        }
    }

    pub fn text_message(from: ShortId, text: String) -> Self {
        Self::new(
            ChatType::Message,
            Some(text.clone()),
            from,
            ChatMessageType::Text,
            None,
            Some(text),
            None,
            Utc::now(),
        )
    }

    pub fn file_mssg(from: ShortId, file: String, r#type: ChatMessageType) -> Self {
        Self::new(
            ChatType::Message,
            None,
            from,
            r#type,
            Some(file),
            None,
            None,
            Utc::now(),
        )
    }

    pub fn announce(text: String) -> Self {
        Self::new(
            ChatType::Announcement,
            Some(text.clone()),
            SYSTEM_ID.to_string(),
            ChatMessageType::Text,
            None,
            Some(text),
            None,
            Utc::now(),
        )
    }
}
