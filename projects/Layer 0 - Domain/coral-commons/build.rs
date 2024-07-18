fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = [
        "src/protos/models.proto",
        "src/protos/Match.proto",
        "src/protos/RequestMatch.proto",
        "src/protos/SetInQueue.proto",
    ];
    let includes = ["src/protos"];

    let mut config = prost_build::Config::new();
    // config
    //     .message_attribute(".models", "#[derive(serde::Serialize, serde::Deserialize)]")
    //     .field_attribute("id", "#[serde(rename = \"_id\")]");

    prost_reflect_build::Builder::new()
        .descriptor_pool("crate::DESCRIPTOR_POOL")
        .compile_protos_with_config(config, &protos, &includes)?;

    println!("Coral generated protos");
    Ok(())
}
