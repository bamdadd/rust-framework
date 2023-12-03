use std::collections::HashMap;

#[derive(Clone)]
pub struct  InMemoryDb<T> {
    pub items: HashMap<String, T>,
}

impl<T> InMemoryDb<T> {
    pub fn new() -> Self {
        InMemoryDb {
            items: HashMap::new(),
        }
    }

    pub fn add(&mut self, id: String,  item: T) {
        self.items.insert(id, item);
    }

    pub fn get(&self, id: &String) -> Option<&T> {
        self.items.get(id)
    }

    pub fn get_all(&self) -> Vec<&T> {
        self.items.values().collect()
    }

    pub fn delete(&mut self, id: &String) -> Option<T> {
        self.items.remove(id)
    }

    pub fn update(&mut self, id: &String, item: T) -> Option<T> {
        self.items.insert(id.clone(), item)
    }
}

// Thread safe wrapper around InMemoryDb
// pub type SharedInMemoryDb<T> = Arc<Mutex<InMemoryDb<T>>>;
