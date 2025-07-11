use actix::{Actor, Addr, Context, Message};
use common::{
    id::ShortId,
    types::{
        room::RoomId,
        session::{SessionClaim, UserRoomType},
    },
};
use std::{collections::HashMap, ops::Add};

use crate::websocket::models::{connection::WsConnection, outgoing::RoomMember};

#[derive(Clone, Debug)]
pub struct Lobby {
    pub rooms: HashMap<RoomId, HashMap<ShortId, (Addr<WsConnection>, RoomMember)>>,
}

#[derive(Message, Debug, Clone)]
#[rtype(result = "()")]
pub struct Connect {
    pub id: ShortId,
    pub room: String,
    pub connection: SessionClaim,
    pub addr: Addr<WsConnection>,
    pub role: UserRoomType,
}

#[derive(Message, Debug, Clone)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: ShortId,
    pub room: String,
    pub session: SessionClaim,
}

impl Lobby {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
        }
    }
}

impl Actor for Lobby {
    type Context = Context<Self>;
}
