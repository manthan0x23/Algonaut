use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use std::fmt;

use crate::utils::web::response::ApiResponse;

#[derive(Debug)]
pub struct AppError {
    status_code: StatusCode,
    message: String,
}

impl AppError {
    pub fn new(status_code: StatusCode, message: &str) -> Self {
        Self {
            status_code,
            message: message.to_string(),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.status_code, self.message)
    }
}

impl std::error::Error for AppError {}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn error_response(&self) -> HttpResponse {
        let body = ApiResponse::<()>::err(&self.message, vec![self.message.clone()]);
        body.respond(self.status_code)
    }
}

#[warn(unused_assignments)]
impl AppError {
    pub fn bad_request(message: &str) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message)
    }

    pub fn unauthorized(message: &str) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, message)
    }

    pub fn forbidden(message: &str) -> Self {
        Self::new(StatusCode::FORBIDDEN, message)
    }

    pub fn not_found(message: &str) -> Self {
        Self::new(StatusCode::NOT_FOUND, message)
    }

    pub fn conflict(message: &str) -> Self {
        Self::new(StatusCode::CONFLICT, message)
    }

    pub fn unprocessable_entity(message: &str) -> Self {
        Self::new(StatusCode::UNPROCESSABLE_ENTITY, message)
    }

    pub fn too_many_requests(message: &str) -> Self {
        Self::new(StatusCode::TOO_MANY_REQUESTS, message)
    }

    pub fn internal_server_error(message: &str) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, message)
    }

    pub fn service_unavailable(message: &str) -> Self {
        Self::new(StatusCode::SERVICE_UNAVAILABLE, message)
    }

    pub fn gateway_timeout(message: &str) -> Self {
        Self::new(StatusCode::GATEWAY_TIMEOUT, message)
    }
}
