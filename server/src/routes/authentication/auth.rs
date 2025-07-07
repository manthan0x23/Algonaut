use actix_web::web::{self, scope};

use crate::routes::authentication::handlers::{callback, login};

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("google").service(login).service(callback));
}
