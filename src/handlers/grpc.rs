use rust_proxy::proxy_server::{Proxy as HandlerTrait, ProxyServer};
use rust_proxy::{AddRouteRequest, AddRouteResponse, HelloReply, HelloRequest};

use tonic::async_trait;
use tonic::{transport::Server, Request, Response, Status};

use std::sync::Arc;

pub mod rust_proxy {
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

        println!("Saying hello to {}", req.name);

        let reply = rust_proxy::HelloReply {
            message: format!("Hello {}!", req.name),
        };

        Ok(Response::new(reply))
    }
    async fn add_route(
        &self,
        request: Request<AddRouteRequest>,
    ) -> Result<Response<AddRouteResponse>, Status> {
        let req = request.into_inner();

        self.ctrl.add(req.from, req.to);

        let response = rust_proxy::AddRouteResponse {};

        Ok(Response::new(response))
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
