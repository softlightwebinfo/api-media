use actix_web::{web};
use crate::app::api::users;

pub fn setup_routes(cfg: &mut web::ServiceConfig) -> &mut web::ServiceConfig {
    cfg
        .service((
            users::list,
        ))
}
