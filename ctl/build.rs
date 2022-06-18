fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .type_attribute(
            "proxy.ListContainersResponse",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "proxy.Container",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .compile(&["../proto/proxy.proto"], &["../proto"])?;

    Ok(())
}
