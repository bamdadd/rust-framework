use serde::{Deserialize, Serialize};
use crate::models::has_id::HasId;

#[derive(Serialize, Deserialize, paperclip::actix::Apiv2Schema, Debug, Clone) ]
pub struct User {
    pub id: Option<String>,
    pub name: String,
}
impl HasId for User {
    fn get_id(&self) -> String {
        self.id.clone().unwrap()
    }

    fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }
}

// impl TypeName for User {
//     fn get_type_name() -> String {
//         "User".to_string()
//     }
//
// }