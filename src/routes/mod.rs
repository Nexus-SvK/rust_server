use actix_web::web;
mod api;

use api::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(handlers::get_users))
            .route("", web::post().to(handlers::create_user)),
    )
    .service(
        web::scope("/products")
            .route("", web::get().to(handlers::get_products))
            .route("", web::post().to(handlers::create_product)),
    );
}
