use crate::utils::{app_state::AppState, web::errors::AppError};
use actix_web::{
    cookie::{Cookie, SameSite}, get, web::{Data, Query}, HttpRequest, HttpResponse, Responder
};
use common::{
    id::short_id,
    types::session::{SessionClaim, UserPayload},
};
use database::entity;
use reqwest::Client;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct CallbackQuery {
    pub code: String,
}

#[get("callback")]
pub async fn callback(
    req: HttpRequest,
    query: Query<CallbackQuery>,
    app_state: Data<AppState>,
) -> impl Responder {
    let db = &app_state.database;
    let redis_pool = &app_state.redis_pool;
    let code = &query.code;

    if code.is_empty() {
        return Err(AppError::bad_request("No code provided"));
    }

    let ip_address = req
        .peer_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let origin = format!(
        "{}://{}",
        req.connection_info().scheme(),
        req.connection_info().host()
    );
    let redirect_uri = format!("{}/api/auth/google/callback", origin);

    let client = Client::new();
    let params = [
        ("code", code.as_str()),
        ("client_id", &app_state.env.google_auth_client_id),
        ("client_secret", &app_state.env.google_auth_client_secret),
        ("redirect_uri", &redirect_uri),
        ("grant_type", "authorization_code"),
    ];

    let token_res = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await
        .map_err(|e| AppError::internal_server_error(&format!("Token exchange failed: {}", e)))?;

    if !token_res.status().is_success() {
        return Err(AppError::bad_request("Google token exchange failed"));
    }

    let token_data: Value = token_res
        .json()
        .await
        .map_err(|e| AppError::internal_server_error(&format!("Token parse failed: {}", e)))?;

    let id_token = token_data
        .get("id_token")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::bad_request("Missing id_token"))?;

    let verify_res = client
        .get(format!(
            "https://oauth2.googleapis.com/tokeninfo?id_token={}",
            id_token
        ))
        .send()
        .await
        .map_err(|e| AppError::internal_server_error(&format!("Verify failed: {}", e)))?;

    if !verify_res.status().is_success() {
        return Err(AppError::bad_request("Google token verify failed"));
    }

    let payload: Value = verify_res
        .json()
        .await
        .map_err(|e| AppError::internal_server_error(&format!("Payload parse failed: {}", e)))?;

    let email = payload
        .get("email")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::bad_request("Missing email"))?;

    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("No Name");

    let picture = payload
        .get("picture")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let user = match entity::user::Entity::find()
        .filter(entity::user::Column::Email.eq(email))
        .one(db)
        .await
        .map_err(|e| AppError::internal_server_error(&format!("DB error: {}", e)))?
    {
        Some(user) => user,
        None => {
            // Create new user
            let active = entity::user::ActiveModel {
                id: Set(short_id(None)),
                email: Set(email.to_string()),
                name: Set(Some(name.to_string())),
                avatar_url: Set(Some(picture.to_string())),
                credits: Set(50),
                ..Default::default()
            };

            active
                .insert(db)
                .await
                .map_err(|e| AppError::internal_server_error(&format!("DB error: {}", e)))?
        }
    };

    let user_payload = UserPayload {
        name: user.name.clone(),
        email: user.email.clone(),
        avatar_url: user.avatar_url.clone(),
        credits: user.credits,
    };

    let session = SessionClaim::new(user.id.clone(), user_payload, ip_address);

    let session_id = redis::session::create_user_session(redis_pool, &session, None)
        .await
        .map_err(|_| AppError::internal_server_error("Couldn't create session"))?;

    let cookie = Cookie::build("sessionId", session_id)
        .path("/")
        .http_only(true)
        .secure(app_state.env.cargo_env == "production")
        .same_site(SameSite::None)
        .finish();

    let client_url = &app_state.env.client_url;
    let mut response = {
        let mut res = HttpResponse::TemporaryRedirect();
        res.append_header(("Location", client_url.to_string()));
        res.cookie(cookie);
        res
    };

    Ok(response.finish())
}
