use crate::websocket::models::{
    lobby::{HandleCrdtUpdate, Lobby},
    outgoing::OutgoingCrdtUpdate,
};
use actix::Handler;
use log::debug;
use tracing::warn;

impl Handler<HandleCrdtUpdate> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: HandleCrdtUpdate, _ctx: &mut Self::Context) -> Self::Result {
     
        let HandleCrdtUpdate {
            update,
            sender,
            room_id,
        } = msg;

        let mut entry = match self.rooms.get_mut(&room_id) {
            Some(room) => room,
            None => return (),
        };

        let (members, doc) = entry;

        if !members.contains_key(&sender.id) {
            log::warn!("Unknown sender for CRDT update");
            return;
        }

        {
            let mut doc = doc.lock().unwrap();

            let changes = update
                .iter()
                .map(|bytes| automerge::Change::from_bytes(bytes.clone()).unwrap())
                .collect::<Vec<_>>();

            if let Err(e) = doc.apply_changes(changes) {
                log::error!("Failed to apply CRDT update: {:?}", e);
                return;
            }
        }

        let message = OutgoingCrdtUpdate::new(update.clone());

        let outgoing_json = match serde_json::to_string(&message) {
            Ok(json) => json,
            Err(e) => {
                warn!("Failed to serialize OutgoingCrdtUpdate: {:?}", e);
                return;
            }
        };

        debug!("updating:: {} members", members.len() - 1);

        for entry in members.iter() {
            let (user_id, (addr, _)) = entry;
            if user_id != &sender.id {
                addr.do_send(crate::websocket::models::WsText(outgoing_json.clone()));
            }
        }
    }
}
