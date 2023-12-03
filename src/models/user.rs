use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, paperclip::actix::Apiv2Schema, Debug, Clone) ]
pub struct User {
    pub id: Option<String>,
    pub name: String,

}
