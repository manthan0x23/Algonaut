use actix_web::{HttpResponse, web};
use common::types::session::{SessionClaim, UserRoomType};
use database::entity::{
    room as Room, sea_orm_active_enums::RoomScopeTypeEnum, user as User, user_room as UserRoom,
};
use reqwest::StatusCode;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QuerySelect,
    TransactionTrait,
};
use serde::Deserialize;
use validator::Validate;

use crate::utils::{
    app_state::AppState, validator::validate_or_bad_request, web::errors::AppError,
    web::response::ApiResponse,
};
use common::id::short_id;

#[derive(Deserialize, Debug, Validate)]
pub struct JoinRoomInput {
    #[validate(length(min = 8, max = 8, message = "room_id must be exactly 8 characters"))]
    pub room_id: String,
}

pub async fn join_room(
    app_data: web::Data<AppState>,
    join_room_input: web::Json<JoinRoomInput>,
    session: SessionClaim,
) -> Result<HttpResponse, AppError> {
    let input = join_room_input.into_inner();
    validate_or_bad_request(&input)?;
    let db = &app_data.database;
    let uid = session.uid.clone();

    let tx = db
        .begin()
        .await
        .map_err(|_| AppError::internal_server_error("Failed to begin transaction"))?;

    let user = User::Entity::find()
        .filter(User::Column::Id.eq(uid.clone()))
        .lock_exclusive()
        .one(&tx)
        .await
        .map_err(|_| AppError::internal_server_error("Error loading user"))?
        .ok_or_else(|| AppError::not_found("User not found"))?;

    let room = Room::Entity::find()
        .filter(Room::Column::Id.eq(input.room_id.clone()))
        .one(&tx)
        .await
        .map_err(|_| AppError::internal_server_error("Error loading room"))?
        .ok_or_else(|| AppError::not_found("Room not found"))?;

    let mut user_room_type = UserRoomType::Viewer;

    match room.editors_scope_type {
        RoomScopeTypeEnum::Open => {
            user_room_type = UserRoomType::Editor;
        }
        RoomScopeTypeEnum::Strict => {
            let allowed: Vec<String> = room.allowed_editors.clone();
            if allowed.contains(&user.email) {
                user_room_type = UserRoomType::Editor;
            }
        }
    };

    match room.viewers_scope_type {
        RoomScopeTypeEnum::Open => {}
        RoomScopeTypeEnum::Strict => {
            let allowed: Vec<String> = room.allowed_viewers.clone();
            if !allowed.contains(&user.id)
                && !allowed.contains(&user.email)
                && (user_room_type == UserRoomType::Viewer)
            {
                return Err(AppError::forbidden("You are not allowed to join this room"));
            }
        }
    }

    let existing = UserRoom::Entity::find()
        .filter(UserRoom::Column::UserId.eq(uid.clone()))
        .filter(UserRoom::Column::RoomId.eq(room.id.clone()))
        .one(&tx)
        .await
        .map_err(|_| AppError::internal_server_error("Error checking existing join"))?;
    if existing.is_some() {
        return Err(AppError::conflict("Already joined this room"));
    }

    let new_user_room = UserRoom::ActiveModel {
        id: Set(short_id(None)),
        user_id: Set(uid.clone()),
        room_id: Set(room.id.clone()),
        r#type: Set(user_room_type.to_string()),
        ..Default::default()
    };

    new_user_room
        .insert(&tx)
        .await
        .map_err(|_| AppError::internal_server_error("Error creating join record"))?;

    tx.commit()
        .await
        .map_err(|_| AppError::internal_server_error("Failed to commit transaction"))?;

    let resp: ApiResponse<String> = ApiResponse::ok("Joined room successfully", room.id);
    Ok(resp.respond(StatusCode::CREATED))
}
