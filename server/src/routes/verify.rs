use crate::utils::web::{errors::AppError, response::ApiResponse};
use actix_web::{HttpResponse, http::StatusCode};
use common::types::session::SessionClaim;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
struct VerifyResponse {
    #[serde(flatten)]
    session: SessionClaim,
}

pub async fn verify(session: SessionClaim) -> Result<HttpResponse, AppError> {
    let response: ApiResponse<VerifyResponse> =
        ApiResponse::ok("Session verified", VerifyResponse { session });

    Ok(response.respond(StatusCode::OK))
}
