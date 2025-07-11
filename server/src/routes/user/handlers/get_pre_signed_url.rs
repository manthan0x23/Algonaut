use actix_web::{HttpResponse, web};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::utils::{
    app_state::AppState,
    validator::validate_or_bad_request,
    web::{errors::AppError, response::ApiResponse},
};

#[derive(Deserialize, Validate)]
pub struct GetPresignedUrlResponseQuery {
    #[validate(length(min = 1))]
    key: String,
}

/// aws s3 `presigned_url` timeout (in seconds)
#[allow(dead_code)]
const PRESIGNED_URL_EXPIRE_TIMEOUT: u64 = 300;

#[derive(Serialize)]
pub struct GetPresignedUrlResponse(String);

pub async fn get_presigned_url(
    query: web::Query<GetPresignedUrlResponseQuery>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let input = query.into_inner();

    validate_or_bad_request(&input)?;

    let storage = &app_state.storage;

    let presigned_url = storage
        .generate_presigned_url(input.key, PRESIGNED_URL_EXPIRE_TIMEOUT)
        .await
        .map_err(|e| {
            AppError::service_unavailable(&format!(
                "Couldn't generate presigned url {}",
                e.to_string()
            ))
        })?;

    let response = GetPresignedUrlResponse(presigned_url);

    let response: ApiResponse<GetPresignedUrlResponse> = ApiResponse::ok(
        &format!(
            "Created presigned url for '{}' seconds",
            PRESIGNED_URL_EXPIRE_TIMEOUT.to_string()
        ),
        response,
    );

    Ok(response.respond(StatusCode::CREATED))
}
