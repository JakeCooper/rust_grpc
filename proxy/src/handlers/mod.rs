mod grpc;

pub async fn new() -> Result<(), Box<dyn std::error::Error>> {
    grpc::init_grpc().await?;
    Ok(())
}
