use std::sync::Mutex;

use actix_web::{HttpResponse, web};
use actix_web::http::StatusCode;
use actix_web::web::Data;
use paperclip::actix::OperationModifier;
use serde::{Deserialize, Serialize};

use crate::db::in_memory::InMemoryDb;
use crate::models;

#[paperclip::actix::api_v2_operation]
pub async fn get_all<T>(data: Data<Mutex<InMemoryDb<T>>>) -> HttpResponse
    where
        T: Serialize + Clone, // Assuming T needs to be serializable and cloneable
{
    let db = match data.lock() {
        Ok(db) => db,
        Err(_) => return HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    };
    HttpResponse::Ok().json(db.get_all())
}


#[paperclip::actix::api_v2_operation]
pub async fn add<T>(
    data: Data<Mutex<InMemoryDb<T>>>,
    mut entity: web::Json<T>,
) -> HttpResponse
    where
        T: Deserialize<'static> + OperationModifier + models::has_id::HasId,
        T: Deserialize<'static> + OperationModifier + models::has_id::HasId,
{
    let mut db = match data.lock() {
        Ok(db) => db,
        Err(_) => return HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let id = uuid::Uuid::new_v4().to_string();
    entity.set_id(id.clone());
    db.add(id, entity.into_inner());

    HttpResponse::Created().json(format!("added"))
}

#[paperclip::actix::api_v2_operation]
pub async fn get_by_id<T>(
    data: web::Data<Mutex<InMemoryDb<T>>>,
    id: web::Path<String>,
) -> HttpResponse
    where
        T: Serialize + OperationModifier + models::has_id::HasId,
{
    let db = match data.lock() {
        Ok(db) => db,
        Err(_) => return HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    };

    match db.get(&id.into_inner()) {
        Some(entity) => HttpResponse::Ok().json(entity),
        None => HttpResponse::NotFound().json(format!("Not found")),
    }
}

#[paperclip::actix::api_v2_operation]
pub async fn delete_by_id<T>(
    data: web::Data<Mutex<InMemoryDb<T>>>,
    id: web::Path<String>,
) -> HttpResponse {
    let mut db = match data.lock() {
        Ok(db) => db,
        Err(_) => return HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    };

    db.delete(&id.into_inner());
    HttpResponse::Ok().json(format!("deleted"))
}

#[paperclip::actix::api_v2_operation]
pub async fn update_by_id<T>(
    data: web::Data<Mutex<InMemoryDb<T>>>,
    id: web::Path<String>,
    item: web::Json<T>,
) -> HttpResponse
    where
        T: Deserialize<'static> + OperationModifier + models::has_id::HasId, {
    let mut db = match data.lock() {
        Ok(db) => db,
        Err(_) => return HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    };
    db.update(&id.into_inner(), item.into_inner());
    HttpResponse::Ok().json(format!("updated"))
}