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

use super::super::controllers::Controller;

// #[derive(Debug, Default)]
pub struct Handler {
    data: Data,
    ctrl: Arc<Controller>,
}

impl Handler {
    fn new() -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
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
        println!("Name: {:?}", req.name);
        println!("{:?}", self.data);

        let name = &req.name;

        // self.clone().ctrl.clone().get_data(); Need to call this function

        println!("Setting Data: {}", req.name.to_string());

        self.ctrl.set_data(req.name.to_string());

        println!("Data: {}", self.ctrl.get_data());

        // *(&self.clone()).ctrl.clone().get_data();

        self.data
            .lock()
            .unwrap()
            .insert(name.to_string(), "VALUE!".into());

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", name.to_string()),
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
