use actix_web::{HttpResponse, web};
use common::types::session::SessionClaim;
use database::entity::{
    room::{self as Room, PAGE_SIZE},
    user_room as UserRoom,
};
use reqwest::StatusCode;
use sea_orm::{
    ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
    QueryTrait,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::utils::{
    app_state::AppState,
    web::{errors::AppError, response::ApiResponse},
};

#[derive(Debug, Deserialize, Validate)]
pub struct GetRoomsQuery {
    #[validate(range(min = 1))]
    pub page: Option<u64>,
    pub only_created: Option<bool>,
    pub only_joined: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct RoomWithRole {
    pub room: Room::Model,
    pub role: String, // "creator" | "editor" | "viewer"
}

#[derive(Serialize, Debug)]
pub struct GetRoomsResponse {
    pub rooms: Vec<RoomWithRole>,
    pub total_items: u64,
    pub total_pages: u64,
}

pub async fn get_rooms_for_user(
    session: SessionClaim,
    app_state: web::Data<AppState>,
    query: web::Query<GetRoomsQuery>,
) -> Result<HttpResponse, AppError> {
    query
        .validate()
        .map_err(|e| AppError::bad_request(&e.to_string()))?;

    let uid = session.uid.clone();
    let email = session.user.email.clone();
    let page = query.page.unwrap_or(1);
    let only_created = query.only_created.unwrap_or(false);
    let only_joined = query.only_joined.unwrap_or(false);

    if only_created && only_joined {
        return Err(AppError::bad_request(
            "cannot set both only_created and only_joined",
        ));
    }

    let cond = {
        let mut c = Condition::any();

        if only_created || (!only_created && !only_joined) {
            c = c.add(Room::Column::CreatedBy.eq(uid.clone()));
        }

        if only_joined || (!only_created && !only_joined) {
            let sub_query = UserRoom::Entity::find()
                .select_only()
                .column(UserRoom::Column::RoomId)
                .filter(UserRoom::Column::UserId.eq(uid.clone()))
                .into_query();
            c = c.add(Room::Column::Id.in_subquery(sub_query));
        }

        c
    };

    let paginator = Room::Entity::find()
        .filter(cond)
        .order_by_desc(Room::Column::CreatedAt)
        .paginate(&app_state.database, PAGE_SIZE);

    let total_items = paginator
        .num_items()
        .await
        .map_err(|e| AppError::internal_server_error(&format!("count error: {}", e)))?;
    let total_pages = paginator
        .num_pages()
        .await
        .map_err(|e| AppError::internal_server_error(&format!("pages error: {}", e)))?;

    let rooms = paginator
        .fetch_page(page.saturating_sub(1))
        .await
        .map_err(|e| AppError::internal_server_error(&format!("fetch error: {}", e)))?;

    let rooms_with_roles: Vec<RoomWithRole> = rooms
        .into_iter()
        .map(|room| {
            let role = if room.created_by == uid {
                "creator"
            } else if room
                .allowed_editors
                .iter()
                .any(|v| v == &email || v == &uid)
            {
                "editor"
            } else {
                "viewer"
            };

            RoomWithRole {
                room,
                role: role.to_string(),
            }
        })
        .collect();

    let json = GetRoomsResponse {
        rooms: rooms_with_roles,
        total_items,
        total_pages,
    };

    let resp: ApiResponse<GetRoomsResponse> = ApiResponse::ok("Fetched rooms", json);
    Ok(resp.respond(StatusCode::OK))
}
