use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::websocket::models::outgoing::ChatMessageType;

// ----------------- INCOMING MESSAGE -----------------

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum IncomingMessage {
    Chat(IncomingChat),
    Ping,
}

// ----------------- INCOMING CHAT -----------------

#[derive(Deserialize, Debug, Clone)]
pub struct IncomingChat {
    pub message_type: ChatMessageType,
    pub url: Option<String>,
    pub text: Option<String>,
    pub _blob: Option<Vec<u8>>,
    pub timestamp: DateTime<Utc>,
}

// ----------------- INCOMING CRDT UPDATE -----------------
#[derive(Deserialize, Debug, Clone)]
pub struct IncommingCrdtUpdate {
    pub update: Vec<Vec<u8>>,
}
