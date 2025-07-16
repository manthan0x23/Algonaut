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
    let db = &app_state.database;
    let room_id_str = path.into_inner();
    let room_id = room_id_str.clone();

    let role = UserRoomEntity::Entity::find()
        .filter(
            Condition::all().add(
                UserRoomEntity::Column::UserId
                    .eq(session.uid.clone())
                    .add(UserRoomEntity::Column::RoomId.eq(room_id)),
            ),
        )
        .one(db)
        .await
        .map_err(|e| {
            AppError::service_unavailable(&format!("Error finding user-room , {}", e.to_string()))
        })?;

    let role = match role {
        Some(r) => UserRoomType(&r.r#type).map_err(|_| {
            AppError::service_unavailable("Invalid user room type")
        })?,
        None => {
            return Err(AppError::not_found(
                "User associated with the room not found or vice versa",
            ));
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
