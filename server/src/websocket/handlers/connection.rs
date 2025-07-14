use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix::{ActorContext, Handler};
use actix_web_actors::ws;
use dashmap::DashMap;
use tracing::debug;

use crate::websocket::models::{
    WsText,
    connection::WsConnection,
    lobby::{Connect, Disconnect, Lobby},
    outgoing::{Broadcast, OutgoingMessage, RoomMember, RoomMembers},
};

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) {
        println!(
            ">> Connect received in Lobby Handler for room: {}",
            msg.room
        );

        let Connect {
            room,
            connection,
            addr,
            role,
        } = msg;

        debug!("Connect is handler {:?} {:?}", room, connection);
        // Insert room if doesn't exist
        let entry = self
            .rooms
            .entry(room.clone())
            .or_insert_with(|| Arc::new(DashMap::new()));

        let users = entry.value();

        let member = RoomMember {
            uid: connection.uid.clone(),
            name: connection.user.name.clone(),
            email: connection.user.email.clone(),
            role: role.clone(),
        };

        users.insert(connection.uid.clone(), (addr.clone(), member.clone()));

        // Broadcast join message and updated member list
        let name = connection
            .user
            .name
            .unwrap_or_else(|| connection.user.email.clone());
        let join_msg =
            OutgoingMessage::Broadcast(Broadcast::announce(format!("{name} joined the space")));
        let member_list = OutgoingMessage::RoomMembers(RoomMembers::announce(
            users.iter().map(|entry| entry.value().1.clone()).collect(),
        ));

        let join_json = serde_json::to_string(&join_msg).unwrap_or_default();
        let members_json = serde_json::to_string(&member_list).unwrap_or_default();

        for ws in users.iter() {
            let _ = ws.value().0.do_send(WsText(join_json.clone()));
            let _ = ws.value().0.do_send(WsText(members_json.clone()));
        }
    }
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, ctx: &mut Self::Context) {
        if let Some(room_entry) = self.rooms.get(&msg.room) {
            let users = room_entry.value();

            let name = msg
                .session
                .user
                .name
                .clone()
                .unwrap_or_else(|| msg.session.user.email.clone());

            if users.remove(&msg.session.uid).is_some() {
                let leave_msg = OutgoingMessage::Broadcast(Broadcast::announce(format!(
                    "{name} left the space"
                )));
                let member_list = OutgoingMessage::RoomMembers(RoomMembers::announce(
                    users.iter().map(|entry| entry.value().1.clone()).collect(),
                ));

                let leave_json = serde_json::to_string(&leave_msg).unwrap_or_default();
                let members_json = serde_json::to_string(&member_list).unwrap_or_default();

                for ws in users.iter() {
                    let _ = ws.value().0.do_send(WsText(leave_json.clone()));
                    let _ = ws.value().0.do_send(WsText(members_json.clone()));
                }
            }

            // Remove room if empty
            if users.is_empty() {
                self.rooms.remove(&msg.room);
            }
        }
    }
}

impl Handler<WsText> for WsConnection {
    type Result = ();

    fn handle(&mut self, msg: WsText, ctx: &mut ws::WebsocketContext<Self>) {
        let _ = ctx.text(msg.0);
    }
}
