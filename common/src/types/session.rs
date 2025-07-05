use crate::id::ShortId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum UserType {
    User,
    Admin,
}

/// Type alias for `session_id`
pub type SessionId = String;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SessionClaim {
    pub iat: u64,
    pub user_type: UserType,
    pub uid: ShortId,
    pub organization_id: Option<ShortId>,
    pub ip: String,
}

impl SessionClaim {
    pub fn new(
        uid: ShortId,
        user_type: UserType,
        organization_id: Option<ShortId>,
        ip: String,
    ) -> Self {
        let iat = Self::now_timestamp();

        Self {
            iat,
            user_type,
            uid,
            organization_id,
            ip,
        }
    }

    fn now_timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }
}
