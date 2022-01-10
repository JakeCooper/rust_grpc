use std::{collections::HashMap, sync::RwLock};

pub type ContentMap = HashMap<String, String>;

pub struct Gateway {
    prefix: String,
    data: RwLock<ContentMap>,
}

impl Gateway {
    pub fn new() -> Self {
        Self {
            prefix: "Hello ".to_string(),
            data: RwLock::new(HashMap::new()),
        }
    }

    pub fn add(&self, from: String, to: String) {
        self.data.write().unwrap().insert(from, to);
    }

    pub fn list(&self) -> ContentMap {
        self.data.read().unwrap().clone()
    }
}
