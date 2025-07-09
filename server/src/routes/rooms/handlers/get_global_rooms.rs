use actix_web::{web, HttpResponse};
use common::types::session::SessionClaim;
use database::entity::{
    room::{self as Room, PAGE_SIZE},
    sea_orm_active_enums::RoomScopeTypeEnum,
};
use reqwest::StatusCode;
use sea_orm::{sqlx::types::chrono, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::utils::{
    app_state::AppState,
    web::{errors::AppError, response::ApiResponse},
};

#[derive(Debug, Deserialize, Validate)]
#[allow(dead_code)]
pub struct GlobalRoomsQuery {
    #[validate(range(min = 1))]
    pub page: Option<u64>,
    pub only_editors: Option<bool>,
    pub only_viewers: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct RoomSummary {
    pub id: String,
    pub alias: String,
    pub objective: String,
    pub editor_open: bool,
    pub viewer_open: bool,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct GetRoomsResponse {
    pub rooms: Vec<RoomSummary>,
    pub total_items: u64,
    pub total_pages: u64,
}

pub async fn get_global_rooms(
    app_state: web::Data<AppState>,
    _session: SessionClaim,
    query: web::Query<GlobalRoomsQuery>,
) -> Result<HttpResponse, AppError> {
    query
        .validate()
        .map_err(|e| AppError::bad_request(&e.to_string()))?;

    let only_editors = query.only_editors.unwrap_or(false);
    let only_viewers = query.only_viewers.unwrap_or(false);
    let page = query.page.unwrap_or(1);

    let mut condition = Condition::any();

    if only_editors {
        condition = condition.add(Room::Column::EditorsScopeType.eq(RoomScopeTypeEnum::Open));
    } else if only_viewers {
        condition = condition.add(Room::Column::ViewersScopeType.eq(RoomScopeTypeEnum::Open));
    } else {
        condition = condition
            .add(Room::Column::EditorsScopeType.eq(RoomScopeTypeEnum::Open))
            .add(Room::Column::ViewersScopeType.eq(RoomScopeTypeEnum::Open));
    }

    let paginator = Room::Entity::find()
        .filter(condition)
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

    let summarized_rooms = rooms
        .into_iter()
        .map(|r| RoomSummary {
            id: r.id,
            alias: r.alias,
            objective: r.objective,
            editor_open: r.editors_scope_type == RoomScopeTypeEnum::Open,
            viewer_open: r.viewers_scope_type == RoomScopeTypeEnum::Open,
            created_by: r.created_by,
            created_at: r.created_at,
        })
        .collect();

    let response = GetRoomsResponse {
        rooms: summarized_rooms,
        total_items,
        total_pages,
    };

    Ok(ApiResponse::ok("Fetched global rooms", response).respond(StatusCode::OK))
}
