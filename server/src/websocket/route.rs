use actix_web::{HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};
use tracing::{error, info};

use crate::utils::{app_state::AppState, web::errors::AppError};
use crate::websocket::models::connection::WsConnection;
use common::types::session::{SessionClaim, UserRoomType};
use database::entity::{room as RoomEntity, user_room as UserRoomEntity};

pub async fn ws_handler(
    path: web::Path<String>,
    req: HttpRequest,
    stream: web::Payload,
    session: SessionClaim,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let room_id_str = path.into_inner();
    let room_id = room_id_str.clone();

    // Lookup room
    let room_model = RoomEntity::Entity::find()
        .filter(RoomEntity::Column::Id.eq(room_id.clone()))
        .one(&app_state.database)
        .await
        .map_err(|e| {
            error!("DB error fetching room {}: {:#?}", room_id, e);
            AppError::internal_server_error("Failed to lookup room")
        })?;

    let room = room_model.ok_or_else(|| {
        error!("Room not found: {}", room_id);
        AppError::not_found("Room does not exist")
    })?;

    // Determine user role
    let role = if room.created_by == session.uid {
        UserRoomType::Creator
    } else {
        // check membership
        let membership = UserRoomEntity::Entity::find()
            .filter(
                Condition::all()
                    .add(UserRoomEntity::Column::RoomId.eq(room.id.clone()))
                    .add(UserRoomEntity::Column::UserId.eq(session.uid.clone())),
            )
            .one(&app_state.database)
            .await
            .map_err(|e| {
                error!(
                    "DB error fetching membership for {} in {}: {:#?}",
                    session.uid, room.id, e
                );
                AppError::internal_server_error("Failed to verify membership")
            })?;

        let membership = membership.ok_or_else(|| {
            error!("Unauthorized: {} tried to join {}", session.uid, room.id);
            AppError::unauthorized("User not associated with the room")
        })?;

        // Map string to enum
        match membership.r#type.as_str() {
            t if t.eq_ignore_ascii_case(&UserRoomType::Viewer.to_string()) => UserRoomType::Viewer,
            _ => UserRoomType::Editor,
        }
    };

    info!(
        "User {} joining room {} as {:?}",
        session.uid, room_id, role
    );

    let ws_conn = WsConnection::new(session, room_id, role, app_state.lobby.clone());
    ws::start(ws_conn, &req, stream).map_err(|e| {
        error!("WebSocket upgrade failed: {:#?}", e);
        AppError::service_unavailable("WebSocket service unavailable")
    })
}
