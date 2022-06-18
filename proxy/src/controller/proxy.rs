use anyhow::Result;

use super::super::gateway::proxy::Gateway as ProxyGateway;

use crate::rust_proxy::{AddRouteRequest, Route};

pub struct Controller {
    pub proxy_gtwy: ProxyGateway,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            proxy_gtwy: ProxyGateway::new(),
        }
    }

    pub async fn add(&self, req: AddRouteRequest) -> Result<String> {
        self.proxy_gtwy.add(req).await
    }

    pub fn list(&self) -> Vec<Route> {
        self.proxy_gtwy.list()
    }

    pub fn remove(&self, uuid: String) {
        self.proxy_gtwy.remove(uuid)
    }
}
