use super::gateway::Gateway;

use crate::rust_proxy::Route;

pub struct Controller {
    pub gtwy: Gateway,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            gtwy: Gateway::new(),
        }
    }

    pub fn add(&self, route: Route) {
        self.gtwy.add(route)
    }

    pub fn list(&self) -> Vec<Route> {
        self.gtwy.list()
    }

    pub fn remove(&self, route: Route) {
        self.gtwy.remove(route)
    }
}
