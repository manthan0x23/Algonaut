use crate::websocket::models::{
    WsText,
    connection::WsConnection,
    lobby::{Connect, Disconnect, Lobby},
    outgoing::{Broadcast, OutgoingMessage, RoomMember, RoomMembers},
};
use actix::Handler;
use actix_web_actors::ws;

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) {
        let room = self.rooms.entry(msg.room.clone()).or_default();

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
            .unwrap_or(msg.connection.user.email);

        let announce_message = format!("{} joined the space", user_name);

        let broadcast = OutgoingMessage::Broadcast(Broadcast::announce(announce_message.clone()));
        let broadcast_json = serde_json::to_string(&broadcast).unwrap_or_default();

        let members = OutgoingMessage::RoomMembers(RoomMembers::announce(
            room.iter()
                .map(|(uid, (_, member))| RoomMember {
                    uid: uid.clone(),
                    name: member.name.clone(),
                    email: member.email.clone(),
                    role: member.role.clone(),
                })
                .collect(),
        ));

        let members_json = serde_json::to_string(&members).unwrap_or_default();

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

                let announce_message = format!("{} left the space", user_name);

                let broadcast: OutgoingMessage =
                    OutgoingMessage::Broadcast(Broadcast::announce(announce_message.clone()));

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

impl Handler<WsText> for WsConnection {
    type Result = ();

    fn handle(&mut self, msg: WsText, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.text(msg.0);
    }
}
