mod controller;
mod gateway;
mod handlers;

pub mod rust_proxy {
    tonic::include_proto!("proxy");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    handlers::new().await?;

    Ok(())
}
