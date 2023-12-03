use actix_web::{App, HttpServer};
use paperclip::actix::{
    OpenApiExt,
    web::{self},
};
use serde::{Deserialize, Serialize};

mod controllers;
mod models;
mod db;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        // Record services and routes from this line.
        .wrap_api()
        // Add routes like you normally do...
        .service(
                web::resource("/users")
                    .route(web::get().to(controllers::users::get_users))
                    .route(web::post().to(controllers::users::add_user)),
        )
        // Or just .service(echo_pet) if you're using the macro syntax
        // Mount the v2/Swagger JSON spec at this path.
        .with_json_spec_at("/api/spec/v2")
        // If you added the "v3" feature, you can also include
        // .with_json_spec_v3_at("/api/spec/v3")

        // ... or if you wish to build the spec by yourself...

        // .with_raw_json_spec(|app, spec| {
        //     app.route("/api/spec", web::get().to(move || {
        //         let spec = spec.clone();
        //         async move {
        //             paperclip::actix::HttpResponseWrapper(actix_web::HttpResponse::Ok().json(&spec))
        //         }
        //     }))
        // })

        // IMPORTANT: Build the app!
        .build()
        .with_swagger_ui()

    ).bind("127.0.0.1:8080")?
        .run().await
}
