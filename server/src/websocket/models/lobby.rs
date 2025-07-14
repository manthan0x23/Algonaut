use actix::{Actor, Addr, Context, Handler, Message, MessageResult};
use automerge;
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
use std::{collections::HashMap, hash::Hash, sync::{Arc, Mutex}};

use crate::{
    utils::app_state::AppEnv,
    websocket::models::{connection::WsConnection, incomming::IncomingChat, outgoing::RoomMember},
};

pub type Room = (
    HashMap<ShortId, (Addr<WsConnection>, RoomMember)>,
    Arc<Mutex<automerge::AutoCommit>>,
);

#[derive(Clone, Debug)]
pub struct Lobby {
    pub rooms: HashMap<RoomId, Room>,
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
        Self {
            rooms: HashMap::new(),
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
