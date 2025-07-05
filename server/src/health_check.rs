use actix_web::{HttpResponse, get, http::StatusCode};
use log::info;

use crate::utils::web::{errors::AppError, response::ApiResponse};

#[get("health-check")]
pub async fn health_check() -> Result<HttpResponse, AppError> {
    info!("health-check success");

    let response: ApiResponse<String> =
        ApiResponse::ok("Server is live", String::from("Health check successfull"));

    Ok(response.respond(StatusCode::OK))
}
