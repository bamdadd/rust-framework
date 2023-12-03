use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, paperclip::actix::Apiv2Schema)]
pub struct User {
    pub id: String,
    pub name: String,
}
