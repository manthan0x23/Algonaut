use actix_web::{HttpResponse, web};
use common::{
    id::short_id,
    types::session::{SessionClaim, UserRoomType},
};
use database::entity::{
    room as Room, sea_orm_active_enums::RoomScopeTypeEnum, user as User, user_room as UserRoom,
};
use reqwest::StatusCode;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
    TransactionTrait,
};
use serde::Deserialize;
use validator::Validate;

use crate::utils::{
    app_state::AppState,
    validator::validate_or_bad_request,
    web::{errors::AppError, response::ApiResponse},
};

#[derive(Debug, Validate, Deserialize, Clone)]
#[allow(dead_code)]
pub struct RoomInput {
    #[validate(length(min = 3, max = 50))]
    pub alias: String,

    pub objective: String,

    pub code_language: Option<String>,

    pub editors_scope_type: Option<RoomScopeTypeEnum>,
    pub viewers_scope_type: Option<RoomScopeTypeEnum>,

    pub allowed_viewers: Option<Vec<String>>,

    pub allowed_editors: Option<Vec<String>>,
}

pub async fn create_room(
    session: SessionClaim,
    room_input: web::Json<RoomInput>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let input = room_input.into_inner();
    let db = &app_state.database;

    validate_or_bad_request(&input)?;

    let txn = db.begin().await.map_err(|e| {
        AppError::internal_server_error(&format!("Failed to start transaction: {}", e))
    })?;

    let user: Option<User::Model> = User::Entity::find()
        .filter(User::Column::Id.eq(session.uid.clone()))
        .lock_exclusive()
        .one(&txn)
        .await
        .map_err(|db_err| {
            AppError::internal_server_error(&format!(
                "Error in database :: {}",
                &db_err.to_string()
            ))
        })?;

    let user = match user {
        Some(u) => u,
        None => {
            return Err(AppError::not_found("User with the session not found !"));
        }
    };

    let rooms_created_by_user = Room::Entity::find()
        .filter(Room::Column::CreatedBy.eq(user.id.clone()))
        .count(db)
        .await
        .map_err(|db_err| {
            AppError::internal_server_error(&format!(
                "Error in database :: {}",
                &db_err.to_string()
            ))
        })?;

    if rooms_created_by_user > 5 && user.credits < 5 {
        return Err(AppError::conflict(
            "Free room limit exceeded, add credits to proceed.",
        ));
    }

    if rooms_created_by_user > 5 {
        let mut user_active: User::ActiveModel = user.clone().into();
        user_active.credits = Set(user.credits - 5);

        User::Entity::update(user_active)
            .exec(&txn)
            .await
            .map_err(|e| {
                AppError::internal_server_error(&format!("Failed to deduct credits: {}", e))
            })?;
    }

    let create_room: Room::ActiveModel = Room::ActiveModel {
        id: Set(short_id(None)),
        alias: Set(input.alias.clone()),
        objective: Set(input.objective),
        code_language: Set(Some(input.code_language.unwrap_or("cpp".to_string()))),
        allowed_editors: Set(input.allowed_editors.unwrap_or_default()),
        allowed_viewers: Set(input.allowed_viewers.unwrap_or_default()),
        editors_scope_type: Set(input.editors_scope_type.unwrap_or(RoomScopeTypeEnum::Open)),
        viewers_scope_type: Set(input.viewers_scope_type.unwrap_or(RoomScopeTypeEnum::Open)),
        created_by: Set(user.id.clone()),
        ..Default::default()
    };

    let create_user_room: UserRoom::ActiveModel = UserRoom::ActiveModel {
        id: Set(short_id(None)),
        user_id: Set(user.id.clone()),
        room_id: create_room.id.clone(),
        r#type: Set(UserRoomType::Creator.to_string()),
        status: Set("joined".to_string()),
    };

    Room::Entity::insert(create_room)
        .exec(&txn)
        .await
        .map_err(|e| AppError::internal_server_error(&format!("Failed to create room: {}", e)))?;

    UserRoom::Entity::insert(create_user_room)
        .exec(&txn)
        .await
        .map_err(|e| {
            AppError::internal_server_error(&format!("Failed to create user_room: {}", e))
        })?;

    txn.commit().await.map_err(|e| {
        AppError::internal_server_error(&format!("Failed to commit transaction: {}", e))
    })?;

    let response: ApiResponse<String> = ApiResponse::ok(
        &format!("Room created successfully {} ", input.alias.clone()),
        input.alias.clone(),
    );

    Ok(response.respond(StatusCode::OK))
}
