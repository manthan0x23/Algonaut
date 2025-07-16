use core::fmt;

use serde::{Deserialize, Serialize};

use crate::id::ShortId;

/// Type alias for `session_id`
pub type SessionId = String;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserPayload {
    pub email: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub credits: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SessionClaim {
    pub iat: u64,
    pub user: UserPayload,
    pub uid: ShortId,
    pub ip: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UserRoomType {
    Creator,
    Viewer,
    Editor,
}


impl UserRoomType {
    
}

impl fmt::Display for UserRoomType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            UserRoomType::Creator => "creator",
            UserRoomType::Viewer => "viewer",
            UserRoomType::Editor => "editor",
        };
        write!(f, "{}", s)
    }



}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserMinimal {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}
