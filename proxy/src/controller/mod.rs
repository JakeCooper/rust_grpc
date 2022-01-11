use anyhow::Result;

use super::gateway::Gateway;

use crate::rust_proxy::{AddRouteRequest, Route};

pub struct Controller {
    pub gtwy: Gateway,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            gtwy: Gateway::new(),
        }
    }

    pub async fn add(&self, req: AddRouteRequest) -> Result<String> {
        self.gtwy.add(req).await
    }

    pub fn list(&self) -> Vec<Route> {
        self.gtwy.list()
    }

    pub fn remove(&self, uuid: String) {
        self.gtwy.remove(uuid)
    }
}
