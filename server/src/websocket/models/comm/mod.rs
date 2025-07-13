pub mod outgoing;
pub mod incomming;

use actix::Message;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsText(pub String);
