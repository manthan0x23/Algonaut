pub mod outgoing;

use actix::Message;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsText(pub String);
