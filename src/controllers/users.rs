use std::sync::Mutex;
use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::http::StatusCode;
use actix_web::web::Data;

use crate::db::in_memory::{InMemoryDb, SharedInMemoryDb};
use crate::models::user::User;

#[paperclip::actix::api_v2_operation]
pub async fn get_users(req: HttpRequest) -> HttpResponse {
    let data = req.app_data::<Data<Mutex<InMemoryDb<User>>>>().unwrap();
    let db = match data.lock() {
        Ok(db) => db,
        Err(_) => return HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    };
    HttpResponse::Ok().json(db.get_all())
}

#[paperclip::actix::api_v2_operation]
pub async fn add_user(
    data: web::Data<Mutex<InMemoryDb<User>>>,
    mut user: web::Json<User>,
) -> HttpResponse {
    let mut db = match data.lock() {
        Ok(db) => db,
        Err(_) => return HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let id = uuid::Uuid::new_v4().to_string();
    user.id.replace(id.clone());
    db.add(id, user.into_inner());


    HttpResponse::Created().json(format!("User added"))
}
