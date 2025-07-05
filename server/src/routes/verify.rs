use crate::utils::{
    app_state::AppState,
    web::{errors::AppError, response::ApiResponse},
};
use actix_web::{HttpRequest, HttpResponse, get, http::StatusCode, web};
use common::types::session::SessionClaim;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
struct VerifyResponse {
    #[serde(flatten)]
    session: SessionClaim,
}

#[get("verify")]
pub async fn verify(
    req: HttpRequest,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let redis_pool = &app_state.redis_pool;

    let session_cookie = req
        .cookie("sessionId")
        .ok_or_else(|| AppError::unauthorized("Missing session cookie"))?;

    let session_id = session_cookie.value();

    let session = redis::session::get_user_session(redis_pool, session_id)
        .await
        .map_err(|e| AppError::internal_server_error(&format!("Redis error: {}", e)))?;

    let session = match session {
        Some(s) => s,
        None => return Err(AppError::not_found("Session not found")),
    };

    let response: ApiResponse<VerifyResponse> =
        ApiResponse::ok("Session verified", VerifyResponse { session });

    Ok(response.respond(StatusCode::OK))
}
