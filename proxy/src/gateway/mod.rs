use std::{collections::HashMap, sync::RwLock};
use tokio::task::JoinHandle;

struct RoutingInfo {
    from: String,
    to: String,
    handle: JoinHandle<()>,
}

use uuid::Uuid;

type RoutingTable = HashMap<Uuid, RoutingInfo>;

use tokio::io;

use tokio::{
    net::{TcpListener, TcpStream},
    select,
};

use crate::rust_proxy::{AddRouteRequest, Route};

pub struct Gateway {
    routing_table: RwLock<RoutingTable>,
}

async fn proxy(client: &str, server: &str) -> io::Result<()> {
    let listener = TcpListener::bind(client).await?;
    loop {
        let (eyeball, _) = listener.accept().await?;
        let origin = TcpStream::connect(server).await?;

        let (mut eread, mut ewrite) = eyeball.into_split();
        let (mut oread, mut owrite) = origin.into_split();

        let e2o = tokio::spawn(async move { io::copy(&mut eread, &mut owrite).await });
        let o2e = tokio::spawn(async move { io::copy(&mut oread, &mut ewrite).await });

        select! {
                _ = e2o => println!("e2o done"),
                _ = o2e => println!("o2e done"),

        }
    }
}

impl Gateway {
    pub fn new() -> Self {
        Self {
            routing_table: RwLock::new(HashMap::new()),
        }
    }

    pub fn add(&self, req: AddRouteRequest) -> String {
        let uuid = uuid::Uuid::new_v4();

        let from = req.from.to_string();
        let to = req.to.to_string();

        let handle = tokio::spawn(async move {
            let from = req.from.to_string();
            let to = req.to.to_string();
            proxy(&from, &to).await.unwrap();
        });

        self.routing_table
            .write()
            .unwrap()
            .insert(uuid, RoutingInfo { from, to, handle });

        uuid.to_string()
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
