use actix_web::web::{self, scope};

use crate::routes::authentication::handlers::{callback, login};

pub fn auth_routes(web_service: &mut web::ServiceConfig) {
    web_service.service(scope("google").service(login).service(callback));
}
