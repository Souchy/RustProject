fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = [
        "src/protos/Match.proto",
        "src/protos/RequestMatch.proto",
        "src/protos/SetInQueue.proto",
    ];
    let includes = ["src/protos"];

    prost_reflect_build::Builder::new()
        .descriptor_pool("crate::DESCRIPTOR_POOL")
        .compile_protos(&protos, &includes)?;

    println!("Coral generated protos");
    Ok(())
}
