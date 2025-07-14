use actix::{Actor, Addr, Context, Message};
use common::{
    id::ShortId,
    storage::AwsS3,
    types::{
        room::RoomId,
        session::{SessionClaim, UserMinimal, UserRoomType},
    },
};
use dashmap::DashMap;
use redis::connect::RedisConnectionPool;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tracing::debug;

use crate::{
    utils::app_state::AppEnv,
    websocket::models::{connection::WsConnection, incomming::IncomingChat, outgoing::RoomMember},
};

pub type Room = Arc<DashMap<ShortId, (Addr<WsConnection>, RoomMember)>>;

pub type SharedRooms = Arc<DashMap<RoomId, Room>>;

#[derive(Clone, Debug)]
pub struct Lobby {
    pub rooms: SharedRooms,
    pub database: DatabaseConnection,
    pub redis_pool: RedisConnectionPool,
    pub env: AppEnv,
    pub storage: AwsS3,
}

#[derive(Message, Debug, Clone)]
#[rtype(result = "()")]
pub struct Connect {
    pub room: String,
    pub connection: SessionClaim,
    pub addr: Addr<WsConnection>,
    pub role: UserRoomType,
}

#[derive(Message, Debug, Clone)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub room: String,
    pub session: SessionClaim,
}

#[derive(Message, Debug, Clone)]
#[rtype(result = "()")]
pub struct HandleChat {
    pub chat_data: IncomingChat,
    pub sender: UserMinimal,
    pub room_id: RoomId,
}

#[derive(Message, Debug, Clone)]
#[rtype(result = "()")]
pub struct HandleCrdtUpdate {
    pub update: Vec<Vec<u8>>,
    pub sender: UserMinimal,
    pub room_id: RoomId,
}

impl Lobby {
    pub fn new(
        db: &DatabaseConnection,
        redis: &RedisConnectionPool,
        env: &AppEnv,
        storage: &AwsS3,
    ) -> Self {
        debug!("Logging lobyy creation");
        Self {
            rooms: Arc::new(DashMap::new()),
            database: db.clone(),
            redis_pool: redis.clone(),
            env: env.clone(),
            storage: storage.clone(),
        }
    }
}

impl Actor for Lobby {
    type Context = Context<Self>;
}
