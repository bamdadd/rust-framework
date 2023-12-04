use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::web::Data;
use paperclip::actix::{OpenApiExt};

use crate::db::in_memory::InMemoryDb;
use crate::models::user::User;

mod controllers;
mod models;
mod db;
mod resources;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data: Data<Mutex<InMemoryDb<User>>> = Data::new(Mutex::new(InMemoryDb::new()));
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|_, _req_head| {
                true
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .max_age(3600);

        let (aggregate_resource, entity_resource) = resources::create_resources::<User>("/users");

        App::new()
            .app_data(Data::clone(&data)) // Share the cloned store
            .wrap_api()
            .wrap(cors)
            .service(aggregate_resource)
            .service(entity_resource)
            .with_json_spec_at("/api-docs/swagger.json")
            .build()
    }
    );
    server.bind("127.0.0.1:8080")?
        .run().await
}
