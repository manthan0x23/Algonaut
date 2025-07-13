use actix_web::{HttpResponse, web};
use common::types::session::{SessionClaim, UserMinimal};
use database::entity::{chat as Chat, user as User, user_room as UserRoom};
use reqwest::StatusCode;
use sea_orm::{
    ColumnTrait, Condition, EntityTrait, JoinType, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::utils::{
    app_state::AppState,
    web::{errors::AppError, response::ApiResponse},
};

#[derive(Debug, Deserialize, Validate)]
pub struct GetRoomChatsQuery {
    #[validate(length(min = 6, max = 64))]
    pub room_id: String,

    #[validate(range(min = 1))]
    pub page: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct ChatWithSender {
    #[serde(flatten)]
    pub chat: Chat::Model,

    pub sender: UserMinimal,
}

#[derive(Debug, Serialize)]
pub struct GetRoomChatsResponse {
    pub chats: Vec<ChatWithSender>,
    pub total_items: u64,
    pub total_pages: u64,
}

pub async fn get_room_chats_for_user(
    session: SessionClaim,
    app_state: web::Data<AppState>,
    query: web::Query<GetRoomChatsQuery>,
) -> Result<HttpResponse, AppError> {
    query
        .validate()
        .map_err(|e| AppError::bad_request(&format!("Invalid query: {}", e)))?;

    let db = &app_state.database;
    let room_id = &query.room_id;
    let page = query.page.unwrap_or(1);

    let user_room = UserRoom::Entity::find()
        .filter(
            Condition::all()
                .add(UserRoom::Column::UserId.eq(session.uid.clone()))
                .add(UserRoom::Column::RoomId.eq(room_id.clone())),
        )
        .one(db)
        .await
        .map_err(|_| AppError::internal_server_error("Error finding room with user"))?;

    if user_room.is_none() {
        return Err(AppError::not_found("User doesn't exists in the room"));
    }

    let paginator = Chat::Entity::find()
        .filter(Chat::Column::RoomId.eq(room_id))
        .order_by_desc(Chat::Column::CreatedAt)
        .paginate(db, Chat::PAGE_SIZE);

    let total_items = paginator
        .num_items()
        .await
        .map_err(|e| AppError::internal_server_error(&format!("Failed to count chats: {}", e)))?;

    let total_pages = paginator
        .num_pages()
        .await
        .map_err(|e| AppError::internal_server_error(&format!("Failed to compute pages: {}", e)))?;

    let chats_with_users = Chat::Entity::find()
        .filter(Chat::Column::RoomId.eq(room_id))
        .order_by_desc(Chat::Column::CreatedAt)
        .join(JoinType::InnerJoin, Chat::Relation::User.def())
        .select_also(User::Entity)
        .paginate(db, Chat::PAGE_SIZE)
        .fetch_page(page.saturating_sub(1))
        .await
        .map_err(|e| AppError::internal_server_error(&format!("Failed to fetch chats: {}", e)))?;

    let chats: Vec<ChatWithSender> = chats_with_users
        .into_iter()
        .map(|(chat, user)| {
            let sender = user.map(|u| UserMinimal {
                id: u.id,
                name: u.name,
                email: Some(u.email),
                avatar_url: u.avatar_url,
            });

            ChatWithSender {
                chat,
                sender: sender.unwrap_or(UserMinimal {
                    id: String::new(),
                    name: None,
                    email: None,
                    avatar_url: None,
                }),
            }
        })
        .collect();

    let response = GetRoomChatsResponse {
        chats,
        total_items,
        total_pages,
    };

    let api_resp: ApiResponse<GetRoomChatsResponse> = ApiResponse::ok("Fetched chats", response);
    Ok(api_resp.respond(StatusCode::OK))
}
