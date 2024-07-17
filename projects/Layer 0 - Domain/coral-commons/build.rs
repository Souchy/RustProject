fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_transport(false)
        .message_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .field_attribute("id", "#[serde(rename = \"_id\")]")
        .compile(
            &[
                "src/protos/objects.proto",
                "src/protos/Match.proto",
                "src/protos/RequestMatch.proto",
                "src/protos/SetInQueue.proto",
            ],
            &["src/protos"],
        )?;
    println!("Realm generated protos");
    Ok(())
}
