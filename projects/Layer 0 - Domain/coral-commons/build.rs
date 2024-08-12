fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = ["src/protos/Match.proto", "src/protos/SetQueue.proto"];
    let includes = ["src/protos"];

    let mut config = prost_build::Config::new();
    config.enum_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, ::rocket::FromFormField)]");
    config.message_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]");
    prost_reflect_build::Builder::new()
        .descriptor_pool("crate::DESCRIPTOR_POOL")
        .compile_protos_with_config(config, &protos, &includes)?;

    println!("Coral generated protos");
    Ok(())
}
