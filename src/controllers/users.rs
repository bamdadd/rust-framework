use uuid::Uuid;
use actix_web::{web, HttpResponse};
use crate::models::user::User;

#[paperclip::actix::api_v2_operation]
pub async fn get_users() -> HttpResponse {
    // Fetch users logic here...
    HttpResponse::Ok().json(vec![User { id: Uuid::new_v4().to_string(), name: "John Doe".to_string() }])
}

#[paperclip::actix::api_v2_operation]
pub async fn add_user(user: web::Json<User>) -> HttpResponse {
    HttpResponse::Created().json(user.into_inner())
}