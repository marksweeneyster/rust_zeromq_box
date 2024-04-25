use std::fs;
use flatc_rust;

use std::path::Path;
use flatc_rust::Flatc;

use protoc_rust;

fn main() {
    println!("cargo:rerun-if-changed=fb_schema/Monster.fbs");

    // @TODO find a portable solution for "flatc" not on PATH
    let flatc = Flatc::from_path("D:/tools/vcpkg/installed/x64-windows/tools/flatbuffers/flatc");

    // First check that we have a good `flatc`
    flatc.check().expect("flatc found");

    flatc.run( flatc_rust::Args {
        inputs: &[Path::new("fb_schema/Monster.fbs")],
        out_dir: Path::new("target/flatbuffers/"),
        ..Default::default()
    }).expect("flatc generated rust file(s)");

    let protos_dir = "target/protos";
    // hmmmm, `protoc_rust` doesn't mkdir (???)
    fs::create_dir_all(protos_dir).expect("Failed to create protos out directory");
    protoc_rust::Codegen::new()
        .protoc_path("D:/tools/vcpkg/installed/x64-windows/tools/protobuf/protoc")
        .out_dir(protos_dir)
        .inputs(&["protobuf_schema/address_book.proto"])
        .include("protobuf_schema")
        .run()
        .expect("Running protoc failed.");
}