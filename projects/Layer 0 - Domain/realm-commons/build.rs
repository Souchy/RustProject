use protobuf_codegen::Codegen;

fn main() {
    // Build protobuf messages refs:
    // https://crates.io/crates/protobuf-codegen/3.4.0
    // https://github.com/stepancheg/rust-protobuf/blob/master/protobuf-examples/customize-serde/build.rs

    Codegen::new()
        .pure()
        .include("src/protos/client")
        .out_dir("src/protos/client/gen")
        .input("src/protos/client/CreateLobby.proto")
        .input("src/protos/client/JoinLobby.proto")
        .input("src/protos/client/SendInvitationToLobby.proto")
        .run_from_script();

    Codegen::new()
        .pure()
        .include("src/protos/server")
        .out_dir("src/protos/server/gen")
        .input("src/protos/server/CreatedLobby.proto")
        .input("src/protos/server/ListLobbies.proto")
        .input("src/protos/server/RelayInvitationToLobby.proto")
        .input("src/protos/server/BroadcastPlayerListInLobby.proto")
        .run_from_script();

    Codegen::new()
        .pure()
        .include("src/protos/models")
        .out_dir("src/protos/models/gen")
        .input("src/protos/models/Lobby.proto")
        .run_from_script();

    println!("Realm generated protos");
}
