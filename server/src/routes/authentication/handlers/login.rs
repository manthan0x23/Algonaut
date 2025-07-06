use crate::utils::{app_state::AppState, web::errors::AppError};
use actix_web::{HttpRequest, HttpResponse, get, web};
use log::debug;

#[get("login")]
pub async fn login(
    req: HttpRequest,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    debug!("Debug google auth");
    let origin = format!(
        "{}://{}",
        req.connection_info().scheme(),
        req.connection_info().host()
    );

    let redirect_uri = format!("{}/api/auth/google/callback", origin);

    let params = [
        (
            "client_id",
            String::from(app_state.env.google_auth_client_id.clone()),
        ),
        ("redirect_uri", redirect_uri),
        ("response_type", String::from("code")),
        ("scope", String::from("email profile openid")),
        ("access_type", String::from("offline")),
        ("prompt", String::from("consent")),
    ];

    let google_auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?{}",
        serde_urlencoded::to_string(&params).unwrap()
    );

    Ok(HttpResponse::Found()
        .append_header(("Location", google_auth_url))
        .finish())
}