use tonic::async_trait;
use tonic::{transport::Server, Request, Response, Status};

use std::sync::Arc;

use crate::rust_proxy::{
    AddRouteRequest, AddRouteResponse, ListContainersRequest, ListContainersResponse,
    ListRoutesRequest, ListRoutesResponse, RemoveRouteRequest, RemoveRouteResponse, VersionRequest,
    VersionResponse,
};

use crate::rust_proxy::proxy_server::{Proxy as HandlerTrait, ProxyServer};

use super::super::controller::{
    containers::Controller as ContainerController, proxy::Controller as ProxyController,
};

// #[derive(Debug)]
pub struct Handler {
    container_ctrl: Arc<ContainerController>,
    proxy_ctrl: Arc<ProxyController>,
}

impl Handler {
    fn new() -> Self {
        Self {
            container_ctrl: Arc::new(ContainerController::new()),
            proxy_ctrl: Arc::new(ProxyController::new()),
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

        match self.proxy_ctrl.add(req).await {
            Ok(uuid) => Ok(Response::new(AddRouteResponse { uuid })),
            Err(e) => Err(Status::invalid_argument(e.to_string())),
        }
    }
    async fn list_routes(
        &self,
        _request: Request<ListRoutesRequest>,
    ) -> Result<Response<ListRoutesResponse>, Status> {
        let routes = self.proxy_ctrl.list();

        let response = ListRoutesResponse { routes };

        Ok(Response::new(response))
    }
    async fn remove_route(
        &self,
        request: Request<RemoveRouteRequest>,
    ) -> Result<Response<RemoveRouteResponse>, Status> {
        let req = request.into_inner();

        self.proxy_ctrl.remove(req.uuid);
        let response = RemoveRouteResponse {};

        return Ok(Response::new(response));
    }
    async fn list_containers(
        &self,
        request: Request<ListContainersRequest>,
    ) -> Result<Response<ListContainersResponse>, Status> {
        let req = request.into_inner();

        let containers = self.container_ctrl.list(req.all).await;

        let response = ListContainersResponse { containers };
        Ok(Response::new(response))
    }
    async fn version(
        &self,
        _request: Request<VersionRequest>,
    ) -> Result<Response<VersionResponse>, Status> {
        let version = self.container_ctrl.version().await;
        let response = VersionResponse { version };
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
