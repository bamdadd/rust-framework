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
                    .route(web::get().to(controllers::crud_controller::get_all::<User>))
                    .route(web::post().to(controllers::crud_controller::add::<User>))
            )
             .service(
                    web::resource("/users/{id}")
                        .route(web::get().to(controllers::crud_controller::get_by_id::<User>))
                        .route(web::delete().to(controllers::crud_controller::delete_by_id::<User>))
                        .route(web::put().to(controllers::crud_controller::update_by_id::<User>))

            )
            .with_json_spec_at("/api-docs/swagger.json")

            .build()
    }
    ).bind("127.0.0.1:8080")?
        .run().await
}
