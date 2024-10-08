fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = [
        "src/protos/models/Lobby.proto",
        "src/protos/models/Player.proto",
        "src/protos/models/User.proto",
        
        "src/protos/server/ListLobbies.proto",
        "src/protos/server/BroadcastPlayerListInLobby.proto",
        "src/protos/server/CreatedLobby.proto",
        "src/protos/server/RelayInvitationToLobby.proto",

        "src/protos/client/Identify.proto",
        "src/protos/client/CreateLobby.proto",
        "src/protos/client/QuitLobby.proto",
        "src/protos/client/JoinLobby.proto",
        "src/protos/client/SendInvitationToLobby.proto",
    ];
    let includes = ["src/protos"];

    let mut config = prost_build::Config::new();
    config.enum_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, ::rocket::FromFormField)]");
    config.message_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]");
    prost_reflect_build::Builder::new()
        .descriptor_pool("crate::DESCRIPTOR_POOL")
        .compile_protos_with_config(config, &protos, &includes)?;

    println!("Realm generated protos");
    Ok(())
}
