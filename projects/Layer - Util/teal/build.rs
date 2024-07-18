fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = ["src/protos/ping.proto", "src/protos/heartbeat.proto"];
    let includes = ["src/protos"];

    let mut config = prost_build::Config::new();
    config
        .message_attribute(".models", "#[derive(serde::Serialize, serde::Deserialize)]")
        .field_attribute("id", "#[serde(rename = \"_id\")]");

    prost_reflect_build::Builder::new()
        .descriptor_pool("crate::DESCRIPTOR_POOL")
        .compile_protos_with_config(config, &protos, &includes)?;

    println!("Realm generated protos");
    Ok(())
}
