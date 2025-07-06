use crate::{
    id::ShortId,
    types::session::{SessionClaim, UserPayload},
};
use actix_web::{FromRequest, HttpMessage, HttpRequest, dev::Payload, error::ErrorUnauthorized};
use std::future::{Ready, ready};

impl SessionClaim {
    pub fn new(uid: ShortId, user: UserPayload, ip: String) -> Self {
        let iat = Self::now_timestamp();

        Self { iat, user, uid, ip }
    }

    fn now_timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }
}

impl FromRequest for SessionClaim {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(
            req.extensions()
                .get::<SessionClaim>()
                .cloned()
                .ok_or(ErrorUnauthorized("Invalid session")),
        )
    }
}
