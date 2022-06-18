use bollard::{container::ListContainersOptions, Docker};

use crate::rust_proxy::Container;

pub struct Gateway {
    docker: Docker,
}

impl Gateway {
    pub fn new() -> Self {
        Self {
            docker: Docker::connect_with_socket_defaults().unwrap(),
        }
    }

    pub async fn version(&self) -> String {
        let v = self.docker.version().await.unwrap();
        v.version.unwrap()
    }

    pub async fn list(&self, all: bool) -> Vec<Container> {
        self.docker
            .list_containers(Some(ListContainersOptions::<String> {
                all,
                ..Default::default()
            }))
            .await
            .unwrap()
            .iter()
            .map(|f| Container {
                id: f.id.clone().unwrap(),
                status: f.state.clone().unwrap(),
                image: f.image.clone().unwrap(),
            })
            .collect()
    }
}
