use protobuf_codegen::Codegen;

fn main() {

    // Build protobuf messages refs:
    // https://crates.io/crates/protobuf-codegen/3.4.0
    // https://github.com/stepancheg/rust-protobuf/blob/master/protobuf-examples/customize-serde/build.rs

    Codegen::new()
        .pure()
        // .cargo_out_dir("messages")
        .include("src/protos")
        .out_dir("src/protos/gen")
        .input("src/protos/messages.proto")
        .input("src/protos/objects.proto")
        .run_from_script();
}
