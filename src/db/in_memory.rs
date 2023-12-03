use std::collections::HashMap;
use crate::models::user::User;

pub struct InMemoryDb {
    users: HashMap<String, User>,
}

impl InMemoryDb {
    pub fn new() -> Self {
        InMemoryDb {
            users: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.id.clone(), user);
    }

    pub fn get_user(&self, id: &String) -> Option<&User> {
        self.users.get(id)
    }
}

