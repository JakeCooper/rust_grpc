mod controller;
mod gateway;
mod handlers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    handlers::new().await?;

    Ok(())
}