use std::fs;
use flatc_rust;

use std::path::Path;
use flatc_rust::Flatc;

use protoc_rust;

fn main() {
    println!("cargo:rerun-if-changed=fb_schema/Monster.fbs");

    let flatc_path = std::env::var("FLATC_DIR").unwrap();

    let flatc = Flatc::from_path(flatc_path);

    // First check that we have a good `flatc`
    flatc.check().expect("flatc found");

    flatc.run( flatc_rust::Args {
        inputs: &[Path::new("fb_schema/Monster.fbs")],
        out_dir: Path::new("target/flatbuffers/"),
        ..Default::default()
    }).expect("flatc generated rust file(s)");

    let protoc_path = std::env::var("PROTOC_DIR").unwrap();
    let protos_dir = "target/protos";
    // hmmmm, `protoc_rust` doesn't mkdir (???)
    fs::create_dir_all(protos_dir).expect("Failed to create protos out directory");
    protoc_rust::Codegen::new()
        .protoc_path(protoc_path)
        .out_dir(protos_dir)
        .inputs(&["protobuf_schema/address_book.proto"])
        .include("protobuf_schema")
        .run()
        .expect("Running protoc failed.");
}