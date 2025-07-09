use crate::{
    utils::{app_state::AppState, web::errors::AppError},
    websocket::models::connection::WsConnection,
};
use actix_web::{HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use common::types::session::{SessionClaim, UserRoomType};
use sea_orm::{ColumnTrait, Condition};

use database::entity::{room as Room, user_room as UserRoom};
use sea_orm::{EntityTrait, QueryFilter};

pub async fn ws_handler(
    path: web::Path<String>,
    req: HttpRequest,
    stream: web::Payload,
    session: SessionClaim,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let db = &app_state.database;
    let room_id = path.into_inner();
    let lobby = &app_state.lobby;

    let role: UserRoomType;

    {
        let room: Option<Room::Model> = Room::Entity::find()
            .filter(Room::Column::Id.eq(room_id.clone()))
            .one(db)
            .await
            .map_err(|_| AppError::internal_server_error("Error processing join"))?;

        let room: Room::Model = match room {
            Some(r) => r,
            None => {
                return Err(AppError::not_found("Room doesn't exists"));
            }
        };

        if room.created_by != session.uid {
            let user_room: Option<UserRoom::Model> = UserRoom::Entity::find()
                .filter(
                    Condition::all()
                        .add(UserRoom::Column::RoomId.eq(room.id))
                        .add(UserRoom::Column::UserId.eq(session.uid.clone())),
                )
                .one(db)
                .await
                .map_err(|_| AppError::internal_server_error("Error processing join"))?;

            let user_room = match user_room {
                Some(u_r) => u_r,
                None => {
                    return Err(AppError::unauthorized("User not associated with the room"));
                }
            };

            if user_room.r#type == UserRoomType::Viewer.to_string() {
                role = UserRoomType::Viewer;
            } else {
                role = UserRoomType::Editor
            }
        } else {
            role = UserRoomType::Creator;
        }
    }

    let connection = WsConnection::new(session, room_id.clone(), role.clone(), lobby.clone());

    ws::start(connection, &req, stream).map_err(|e| {
        AppError::service_unavailable(&format!("Error serving sockets {}", e.to_string()))
    })
}
