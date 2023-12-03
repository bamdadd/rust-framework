use actix_web::{App, http, HttpServer};
use actix_cors::Cors;
use paperclip::actix::{
    OpenApiExt,
    web::{self},
};


mod controllers;
mod models;
mod db;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        let cors =Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                true
            })
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);


        App::new()
            .wrap_api()
            .wrap(cors)
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
