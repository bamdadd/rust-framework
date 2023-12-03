use actix_web::{App, HttpServer};
use paperclip::actix::{
    OpenApiExt,
    web::{self},
};


mod controllers;
mod models;
mod db;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| App::new()
        .wrap_api()
        .service(
                web::resource("/users")
                    .route(web::get().to(controllers::users::get_users))
                    .route(web::post().to(controllers::users::add_user)),
        )
        .with_json_spec_at("/api-docs/openapi.json")

        .build()

    ).bind("127.0.0.1:8080")?
        .run().await
}
