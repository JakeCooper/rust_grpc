use std::collections::HashMap;

use tonic::async_trait;
use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use std::sync::Mutex;

mod bonk;

pub mod hello_world {
    tonic::include_proto!("service");
}

type Data = Mutex<HashMap<String, String>>;

#[derive(Debug, Default)]
pub struct MyGreeter {
    data: Data,
}

impl MyGreeter {
    fn new() -> Self {
        MyGreeter {
            data: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait] // Remove this
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let req = request.into_inner();
        println!("Name: {:?}", req.name);
        println!("{:?}", self.data);

        let name = &req.name;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let greeter = MyGreeter::new();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    println!("Running!");

    Ok(())
}
