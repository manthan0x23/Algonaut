use actix::{ActorContext, Handler, StreamHandler};
use actix_web_actors::ws;
use std::time::Instant;

use crate::websocket::models::{
    WsText,
    connection::WsConnection,
    lobby::{Connect, Disconnect, Lobby},
    outgoing::{Broadcast, OutgoingMessage, RoomMember, RoomMembers},
};

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        let room = self.rooms.entry(msg.room).or_default();

        let new_member = RoomMember {
            name: msg.connection.user.name.clone(),
            email: msg.connection.user.email.clone(),
            role: msg.role.clone(),
            uid: msg.connection.uid.clone(),
        };

        room.insert(msg.connection.uid, (msg.addr.clone(), new_member));

        let user_name = match msg.connection.user.name {
            Some(name) => name,
            None => msg.connection.user.email,
        };

        let broadcast: OutgoingMessage = OutgoingMessage::Broadcast(Broadcast::announce(format!(
            "{} joined the space ",
            user_name
        )));

        let broadcast_json =
            serde_json::to_string(&broadcast).unwrap_or("False broadcast loading".to_string());

        let mut members: Vec<RoomMember> = vec![];

        for (uid, (_, member)) in room.iter() {
            members.push(RoomMember {
                uid: uid.clone(),
                name: member.name.clone(),
                email: member.email.clone(),
                role: member.role.clone(),
            });
        }

        let members = OutgoingMessage::RoomMembers(RoomMembers::announce(members));

        let members_json =
            serde_json::to_string(&members).unwrap_or("False members loading".to_string());

        for (_, (ws_conn, _)) in room.iter() {
            ws_conn.do_send(WsText(broadcast_json.clone()));
            ws_conn.do_send(WsText(members_json.clone()));
        }
    }
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        if let Some(room) = self.rooms.get_mut(&msg.room) {
            if let Some(_) = room.remove(&msg.session.uid) {
                let user_name = match msg.session.user.name {
                    Some(name) => name,
                    None => msg.session.user.email,
                };

                let broadcast: OutgoingMessage = OutgoingMessage::Broadcast(Broadcast::announce(
                    format!("{} left the space ", user_name),
                ));

                let broadcast_json = serde_json::to_string(&broadcast)
                    .unwrap_or("False broadcast loading".to_string());

                let mut members: Vec<RoomMember> = vec![];

                for (uid, (_, member)) in room.iter() {
                    members.push(RoomMember {
                        uid: uid.clone(),
                        name: member.name.clone(),
                        email: member.email.clone(),
                        role: member.role.clone(),
                    });
                }

                let members = OutgoingMessage::RoomMembers(RoomMembers::announce(members));

                let members_json =
                    serde_json::to_string(&members).unwrap_or("False members loading".to_string());

                for (_, (ws_conn, _)) in room.iter() {
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

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConnection {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(ws::Message::Ping(msg)) => {
                self.last_heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.last_heartbeat = Instant::now();
            }
            Ok(ws::Message::Text(_text)) => {}
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Binary(_)) => {
                ctx.text("error: binary not supported");
            }
            _ => {}
        }
    }
}

impl Handler<WsText> for WsConnection {
    type Result = ();

    fn handle(&mut self, msg: WsText, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.text(msg.0);
    }
}
