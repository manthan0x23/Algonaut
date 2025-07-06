use crate::utils::{app_state::AppState, web::errors::AppError};
use actix_web::{
    Error, HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    web,
};
use common::types::session::SessionClaim;

pub async fn auth_middleware(
    req: ServiceRequest,
    next: actix_web::middleware::Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let app_data = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| AppError::internal_server_error("Internal Server Error"))?;

    let session_cookie = req
        .cookie("sessionId")
        .ok_or_else(|| AppError::unauthorized("Missing session cookie"))?;

    let session_id = session_cookie.value();

    let session = redis::session::get_user_session(&app_data.redis_pool, session_id)
        .await
        .map_err(|_| AppError::internal_server_error("Error connecting to Redis"))?;

    let session = session.ok_or_else(|| AppError::forbidden("User not authorized"))?;

    req.extensions_mut().insert::<SessionClaim>(session);

    next.call(req).await
}
