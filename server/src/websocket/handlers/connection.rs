use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix::Handler;
use actix_web_actors::ws;

use crate::websocket::models::{
    WsText,
    connection::WsConnection,
    lobby::{Connect, Disconnect, Lobby},
    outgoing::{Broadcast, OutgoingMessage, RoomMember, RoomMembers},
};

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) {
        let mut entry = self.rooms.entry(msg.room.clone()).or_insert_with(|| {
            let doc = Arc::new(Mutex::new(automerge::AutoCommit::new()));
            (HashMap::new(), doc)
        });

        let (room, _doc) = &mut *entry;

        let new_member = RoomMember {
            name: msg.connection.user.name.clone(),
            email: msg.connection.user.email.clone(),
            role: msg.role.clone(),
            uid: msg.connection.uid.clone(),
        };

        room.insert(
            msg.connection.uid.clone(),
            (msg.addr.clone(), new_member.clone()),
        );

        let user_name = msg
            .connection
            .user
            .name
            .clone()
            .unwrap_or(msg.connection.user.email.clone());

        let announce_message = format!("{} joined the space", user_name);

        let broadcast = OutgoingMessage::Broadcast(Broadcast::announce(announce_message));
        let broadcast_json = serde_json::to_string(&broadcast).unwrap_or_default();

        let members = OutgoingMessage::RoomMembers(RoomMembers::announce(
            room.iter()
                .map(|entry| {
                    let (uid, (_ws_conn, member)) = entry;
                    RoomMember {
                        uid: uid.clone(),
                        name: member.name.clone(),
                        email: member.email.clone(),
                        role: member.role.clone(),
                    }
                })
                .collect(),
        ));

        let members_json = serde_json::to_string(&members).unwrap_or_default();

        for entry in room.iter() {
            let (_, (ws_conn, _)) = entry;
            ws_conn.do_send(WsText(broadcast_json.clone()));
            ws_conn.do_send(WsText(members_json.clone()));
        }
    }
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) {
        if let Some(mut entry) = self.rooms.get_mut(&msg.room) {
            let (room, _doc) = &mut *entry;

            // Remove the user if present
            if room.remove(&msg.session.uid).is_some() {
                let user_name = msg
                    .session
                    .user
                    .name
                    .clone()
                    .unwrap_or(msg.session.user.email.clone());

                let announce_message = format!("{} left the space", user_name);

                let broadcast = OutgoingMessage::Broadcast(Broadcast::announce(announce_message));
                let broadcast_json = serde_json::to_string(&broadcast).unwrap_or_default();

                let members = OutgoingMessage::RoomMembers(RoomMembers::announce(
                    room.iter()
                        .map(|entry| {
                            let (uid, (_, member)) = entry;
                            RoomMember {
                                uid: uid.clone(),
                                name: member.name.clone(),
                                email: member.email.clone(),
                                role: member.role.clone(),
                            }
                        })
                        .collect(),
                ));

                let members_json = serde_json::to_string(&members).unwrap_or_default();

                for entry in room.iter() {
                    let (_, (ws_conn, _)) = entry;
                    ws_conn.do_send(WsText(broadcast_json.clone()));
                    ws_conn.do_send(WsText(members_json.clone()));
                }
            }

            if room.is_empty() {
                self.rooms.remove(&msg.room);
            }
        }
    }
}

impl Handler<WsText> for WsConnection {
    type Result = ();

    fn handle(&mut self, msg: WsText, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.text(msg.0);
    }
}
