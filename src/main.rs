use std::sync::{Arc, Mutex};
use actix_web::{App, HttpServer, middleware::Logger};
use paperclip::actix::{
    OpenApiExt,
    web::{self},
};

use actix_cors::Cors;
use actix_web::web::Data;
use crate::db::in_memory::{InMemoryDb};
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
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);

        App::new()
            .app_data(Data::clone(&data)) // Share the cloned store
            .wrap_api()
            .wrap(cors)
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(
                web::resource("/users")
                    .route(web::get().to(controllers::users::get_users))
                    .route(web::post().to(controllers::users::add_user)),
            )
            .with_json_spec_at("/api-docs/swagger.json")

            .build()
    }
    ).bind("127.0.0.1:8080")?
        .run().await
}
