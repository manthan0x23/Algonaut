use actix::{Actor, Addr, Context, Message};
use common::{
    id::ShortId,
    types::{room::RoomId, session::SessionClaim},
};
use std::{collections::HashMap, ops::Add};

use crate::websocket::models::connection::WsConnection;

#[derive(Clone, Debug)]
pub struct Lobby {
    pub rooms: HashMap<RoomId, HashMap<ShortId, Addr<WsConnection>>>,
}

#[derive(Message, Debug, Clone)]
#[rtype(result = "()")]
pub struct Connect {
    pub id: ShortId,
    pub room: String,
    pub connection: SessionClaim,
    pub addr: Addr<WsConnection>,
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
