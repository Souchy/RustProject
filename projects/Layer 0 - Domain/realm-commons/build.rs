fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_transport(false)
        .message_attribute(".models", "#[derive(serde::Serialize, serde::Deserialize)]")
        .field_attribute("id", "#[serde(rename = \"_id\")]")
        .compile(
            &[
                "src/protos/models/Lobby.proto",
                "src/protos/server/ListLobbies.proto",
            ],
            &["src/protos"],
        )?;
    println!("Realm generated protos");
    Ok(())
}
