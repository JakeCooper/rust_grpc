use tonic::async_trait;
use tonic::{transport::Server, Request, Response, Status};

use std::sync::Arc;

use crate::rust_proxy::{
    AddRouteRequest, AddRouteResponse, ListRoutesRequest, ListRoutesResponse, RemoveRouteRequest,
    RemoveRouteResponse,
};

use crate::rust_proxy::proxy_server::{Proxy as HandlerTrait, ProxyServer};

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
    async fn add_route(
        &self,
        request: Request<AddRouteRequest>,
    ) -> Result<Response<AddRouteResponse>, Status> {
        let req = request.into_inner();

        let response = AddRouteResponse {
            uuid: self.ctrl.add(req),
        };

        Ok(Response::new(response))
    }
    async fn list_routes(
        &self,
        _request: Request<ListRoutesRequest>,
    ) -> Result<Response<ListRoutesResponse>, Status> {
        let routes = self.ctrl.list();

        let response = ListRoutesResponse { route: routes };

        Ok(Response::new(response))
    }
    async fn remove_route(
        &self,
        request: Request<RemoveRouteRequest>,
    ) -> Result<Response<RemoveRouteResponse>, Status> {
        let req = request.into_inner();

        self.ctrl.remove(req.uuid);
        let response = RemoveRouteResponse {};

        return Ok(Response::new(response));
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
