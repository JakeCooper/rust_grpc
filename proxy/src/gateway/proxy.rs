use anyhow::Result;
use std::{collections::HashMap, sync::RwLock};
use tokio::task::JoinHandle;

struct RoutingInfo {
    from: String,
    to: String,
    handle: JoinHandle<()>,
}

use uuid::Uuid;

type RoutingTable = HashMap<Uuid, RoutingInfo>;

use tokio::io::{self, AsyncWriteExt};

use tokio::net::{TcpListener, TcpStream};

use crate::rust_proxy::{AddRouteRequest, Route};

pub struct Gateway {
    routing_table: RwLock<RoutingTable>,
}

async fn transfer(mut inbound: TcpStream, proxy_addr: String) -> Result<()> {
    let mut outbound = TcpStream::connect(proxy_addr).await?;

    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = outbound.split();

    let client_to_server = async {
        io::copy(&mut ri, &mut wo).await?;
        (&mut wo).shutdown().await
    };

    let server_to_client = async {
        io::copy(&mut ro, &mut wi).await?;
        (&mut wi).shutdown().await
    };

    tokio::try_join!(client_to_server, server_to_client)?;

    Ok(())
}

async fn proxy(listener: &TcpListener, server: &str) -> Result<()> {
    while let Ok((inbound, _)) = listener.accept().await {
        let transfer = transfer(inbound, server.to_string());

        tokio::spawn(transfer);
    }

    Ok(())
}

impl Gateway {
    pub fn new() -> Self {
        Self {
            routing_table: RwLock::new(HashMap::new()),
        }
    }

    pub async fn add(&self, req: AddRouteRequest) -> Result<String> {
        let uuid = uuid::Uuid::new_v4();

        let listener = TcpListener::bind(&req.from).await?;
        let to = req.to.clone();

        let handle = tokio::spawn(async move {
            proxy(&listener, &to).await.unwrap();
        });

        self.routing_table.write().unwrap().insert(
            uuid,
            RoutingInfo {
                from: req.from,
                to: req.to,
                handle,
            },
        );

        Ok(uuid.to_string())
    }

    pub fn list(&self) -> Vec<Route> {
        self.routing_table
            .read()
            .expect("Poisoned Read!")
            .iter()
            .map(|(k, routing_info)| Route {
                from: routing_info.from.to_string(),
                to: routing_info.to.to_string(),
                uuid: k.to_string(),
            })
            .collect()
    }

    pub fn remove(&self, uuid: String) {
        self.routing_table
            .read()
            .unwrap()
            .get(&Uuid::parse_str(&uuid).unwrap())
            .unwrap()
            .handle
            .abort();

        self.routing_table
            .write()
            .unwrap()
            .remove(&Uuid::parse_str(&uuid).unwrap());
    }
}
