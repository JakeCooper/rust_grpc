use std::vec;

use tokio::time::{sleep, Duration};

use hello_world::HelloRequest;
use hello_world::{greeter_client::GreeterClient, HelloReply};

use futures::future::join_all;

use tonic::{Request, Response, Status};

pub mod hello_world {
    tonic::include_proto!("service");
}

async fn do_req(i: u64) -> Result<Response<HelloReply>, Status> {
    let mut client = GreeterClient::connect("http://0.0.0.0:50051")
        .await
        .unwrap();
    let request = Request::new(HelloRequest {
        name: format!("Jake {}", i),
    });
    client.say_hello(request).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tokio::spawn(async move {
        let mut cnt = 0;
        loop {
            let mut futures = vec![];
            for value in 0..cnt {
                let task = tokio::spawn(do_req(value));
                futures.push(task);
            }
            join_all(futures).await;
            cnt += 1;
            let cycle_time = cnt * 1000;
            println!("Bumped cycle time to {}", cycle_time);
            sleep(Duration::from_millis(cycle_time)).await;
        }
    })
    .await;

    println!("Never");

    Ok(())
}
