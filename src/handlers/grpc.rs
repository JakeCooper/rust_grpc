use hello_world::proxy_server::{Proxy as HandlerTrait, ProxyServer};
use hello_world::{HelloReply, HelloRequest};

use std::collections::HashMap;

use tonic::async_trait;
use tonic::{transport::Server, Request, Response, Status};

use std::sync::{Arc, Mutex};

pub mod hello_world {
    tonic::include_proto!("proxy");
}

use super::super::controller::Controller;

// #[derive(Debug)]
pub struct Handler {
    ctrl: Arc<Controller>,
}

impl Handler {
    fn new() -> Self {
        Self {
            ctrl: Arc::new(Controller::new()),
        }
    }
}

#[async_trait]
impl HandlerTrait for Handler {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let req = request.into_inner();

        self.ctrl.mutate(req.name.to_string());
        println!("{}", self.ctrl.read());

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", req.name),
        };

        Ok(Response::new(reply))
    }
}

pub async fn init_grpc() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let greeter = Handler::new();

    Server::builder()
        .add_service(ProxyServer::new(greeter))
        .serve(addr)
        .await?;

    println!("Running!");
    Ok(())
}
