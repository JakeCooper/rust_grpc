mod controllers;
mod gateways;
mod handlers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    handlers::new().await?;

    Ok(())
}
