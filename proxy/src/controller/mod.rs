use crate::gateway::ContentMap;

use super::gateway::Gateway;

pub struct Controller {
    pub gtwy: Gateway,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            gtwy: Gateway::new(),
        }
    }

    pub fn add(&self, from: String, to: String) {
        self.gtwy.add(from, to)
    }

    pub fn read(&self) -> ContentMap {
        self.gtwy.list()
    }
}
