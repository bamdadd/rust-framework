use paperclip::actix::web::Resource;
use serde::{Deserialize, Serialize};
use paperclip::actix::{OperationModifier, web};
use crate::{controllers, models};

pub fn create_resources<T>(path: &str) -> (Resource, Resource)
    where
        T: Clone + Serialize + for<'de> Deserialize<'de> + 'static + OperationModifier + models::has_id::HasId, {
    let aggregate_resource = create_aggregate_resource::<T>(path);
    let entity_resource = create_entity_resource::<T>(&*(path.to_owned() + "/{id}"));
    (aggregate_resource, entity_resource)
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
