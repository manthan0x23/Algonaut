use std::time::Duration;

use common::{
    id::{ShortId, short_id},
    types::session::{SessionClaim, SessionId},
};
use deadpool_redis::{Pool, redis::AsyncCommands};
use serde_json;

use crate::connect::connect_via_pool;

fn generate_session_key_from_id(id: &str) -> SessionId {
    format!("algonaut:session:{}", id)
}

pub async fn get_user_session(
    pool: &Pool,
    session_id: &str,
) -> Result<Option<SessionClaim>, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = connect_via_pool(pool).await?;

    let session_key: SessionId = generate_session_key_from_id(session_id);

    let session_json: Option<String> = conn.get(session_key).await?;

    match session_json {
        Some(json) => {
            let session: SessionClaim = serde_json::from_str(&json)
                .map_err(|e| format!("Failed to parse session JSON: {}", e))?;
            Ok(Some(session))
        }
        None => Ok(None),
    }
}

pub async fn create_user_session(
    pool: &Pool,
    session: &SessionClaim,
    ttl: Option<Duration>,
) -> Result<ShortId, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = connect_via_pool(pool).await?;

    let session_id = short_id(Some(10));
    let session_key: SessionId = generate_session_key_from_id(&session_id);

    let session_json = serde_json::to_string(session)
        .map_err(|e| format!("Failed to serialize SessionClaim: {}", e))?;

    let expire_seconds: u64 = ttl
        .unwrap_or_else(|| Duration::from_secs(24 * 60 * 60))
        .as_secs() as u64;

    conn.set_ex::<_, _, ()>(session_key, session_json, expire_seconds)
        .await?;

    Ok(session_id)
}
