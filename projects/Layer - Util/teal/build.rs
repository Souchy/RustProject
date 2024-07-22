fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = ["src/protos/ping.proto", "src/protos/heartbeat.proto"];
    let includes = ["src/protos"];

    let config = prost_build::Config::new();

    prost_reflect_build::Builder::new()
        .descriptor_pool("crate::DESCRIPTOR_POOL")
        .compile_protos_with_config(config, &protos, &includes)?;

    println!("Realm generated protos");
    Ok(())
}
