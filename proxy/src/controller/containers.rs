use super::super::gateway::docker::Gateway as DockerGateway;

use crate::rust_proxy::Container;

pub struct Controller {
    pub docker_gtwy: DockerGateway,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            docker_gtwy: DockerGateway::new(),
        }
    }

    pub async fn list(&self, all: bool) -> Vec<Container> {
        let containers = self.docker_gtwy.list(all).await;
        println!("{:?}", containers);
        containers
    }

    pub async fn version(&self) -> String {
        self.docker_gtwy.version().await
    }
}
