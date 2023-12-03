use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::web::Data;
use paperclip::actix::{
    OpenApiExt,
    web::{self},
};

use crate::db::in_memory::InMemoryDb;
use crate::models::user::User;

mod controllers;
mod models;
mod db;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let data: Data<Mutex<InMemoryDb<User>>> = Data::new(Mutex::new(InMemoryDb::new()));
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                true
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .max_age(3600);

        App::new()
            .app_data(Data::clone(&data)) // Share the cloned store
            .wrap_api()
            .wrap(cors)
            .service(
                web::resource("/users")
                    .route(web::get().to(controllers::users::get_users))
                    .route(web::post().to(controllers::users::add_user)))
             .service(
                    web::resource("/users/{id}")
                        .route(web::get().to(controllers::users::get_user_by_id))
                        .route(web::delete().to(controllers::users::delete_user_by_id))
                        .route(web::put().to(controllers::users::update_user_by_id))

            )
            .with_json_spec_at("/api-docs/swagger.json")

            .build()
    }
    ).bind("127.0.0.1:8080")?
        .run().await
}
