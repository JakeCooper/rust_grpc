use hello_world::greeter_server::{Greeter as HandlerTrait, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

use std::collections::HashMap;

use tonic::async_trait;
use tonic::{transport::Server, Request, Response, Status};

use std::sync::{Arc, Mutex};

pub mod hello_world {
    tonic::include_proto!("service");
}

type Data = Mutex<HashMap<String, String>>;

use super::super::gateway::Gateway;

// #[derive(Debug)]
pub struct Handler {
    data: Data,
    ctrl: Arc<Gateway>,
}

impl Handler {
    fn new() -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
            ctrl: Arc::new(Gateway::new()),
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
        println!("Name: {:?}", req.name);
        println!("{:?}", self.data);

        // self.clone().ctrl.clone().get_data(); Need to call this function

        println!("Setting Data: {}", req.name);

        self.ctrl.mutate(req.name.to_string());

        self.ctrl.read();

        self.data
            .lock()
            .unwrap()
            .insert(req.name.to_string(), "VALUE!".into());

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
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    println!("Running!");
    Ok(())
}
