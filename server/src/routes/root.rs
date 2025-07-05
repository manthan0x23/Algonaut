use actix_web::web::{self, scope};

use crate::routes::verify;

pub fn app_root(web_service: &mut web::ServiceConfig) {
    web_service.service(verify);
}
