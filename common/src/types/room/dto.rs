use serde::{Deserialize, Serialize};
use validator::ValidationError;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RoomScopeType {
    Open,
    Strict,
}

pub fn validate_room_scope_type(val: &RoomScopeType) -> Result<(), ValidationError> {
    match val {
        RoomScopeType::Open | RoomScopeType::Strict => Ok(()),
    }
}

/// Alias for `room_id`
pub type RoomId = String;
