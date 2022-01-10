use std::{collections::HashMap, sync::RwLock};

pub type ContentMap = HashMap<String, String>;

pub use crate::rust_proxy::Route;

pub mod rust_proxy {
    tonic::include_proto!("proxy");
}

pub struct Gateway {
    data: RwLock<ContentMap>,
}

impl Gateway {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
        }
    }

    pub fn add(&self, route: Route) {
        self.data.write().unwrap().insert(route.from, route.to);
    }

    pub fn list(&self) -> Vec<Route> {
        self.data
            .read()
            .expect("Poisoned Read!")
            .iter()
            .map(|(from, to)| Route {
                from: from.to_string(),
                to: to.to_string(),
            })
            .collect()
    }

    pub fn remove(&self, route: Route) {
        self.data.write().unwrap().remove(&route.from);
    }
}
