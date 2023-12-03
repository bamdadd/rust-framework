use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::web::Data;
use paperclip::actix::{OpenApiExt, OperationModifier, web::{self}};
use paperclip::actix::web::Resource;
use serde::{Deserialize, Serialize};

use crate::db::in_memory::InMemoryDb;
use crate::models::user::User;

mod controllers;
mod models;
mod db;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data: Data<Mutex<InMemoryDb<User>>> = Data::new(Mutex::new(InMemoryDb::new()));
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                true
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .max_age(3600);

        let aggregate_resource = create_aggregate_resource::<User>("/users");
        let entity_resource = create_entity_resource::<User>("/users/{id}");
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

fn create_entity_resource<T>(path: &str)
                             -> Resource
    where
        T: Clone + Serialize + for<'de> Deserialize<'de> + 'static + OperationModifier + models::has_id::HasId, {
    web::resource(path)
        .route(web::get().to(controllers::crud_controller::get_by_id::<T>))
        .route(web::delete().to(controllers::crud_controller::delete_by_id::<T>))
        .route(web::put().to(controllers::crud_controller::update_by_id::<T>))
}

fn create_aggregate_resource<T>(path: &str) -> Resource
    where
        T: Clone + Serialize +for<'de> Deserialize<'de> + OperationModifier + models::has_id::HasId + 'static, {
    web::resource(path)
        .route(web::get().to(controllers::crud_controller::get_all::<T>))
        .route(web::post().to(controllers::crud_controller::add::<T>))
}
