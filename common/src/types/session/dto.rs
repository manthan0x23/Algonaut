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
