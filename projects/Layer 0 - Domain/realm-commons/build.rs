use protobuf_codegen::Codegen;

fn main() {

    // Build protobuf messages refs:
    // https://crates.io/crates/protobuf-codegen/3.4.0
    // https://github.com/stepancheg/rust-protobuf/blob/master/protobuf-examples/customize-serde/build.rs
    
    Codegen::new()
        .pure()
        .include("src/protos")
        .out_dir("src/messages")
        .input("src/protos/heartbeat.proto")
        .input("src/protos/example.proto")
        .input("src/protos/ping.proto")
        .input("src/protos/user.proto")
        .run_from_script();
    println!("Hello, world!");
}
