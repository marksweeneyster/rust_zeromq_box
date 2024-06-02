use std::path::Path;

use flatc_rust::Flatc;

fn main() {
    println!("cargo:rerun-if-changed=fb_schema/Monster.fbs");

    let flatc_env_name = std::env::var("FLATC_DIR").unwrap();
    let flatc_path = Path::new(&flatc_env_name);
    let flatc = Flatc::from_path(flatc_path);

    // First check that we have a good `flatc`
    flatc.check().expect("flatc NOT found");

    flatc
        .run(flatc_rust::Args {
            inputs: &[Path::new("fb_schema/Monster.fbs")],
            out_dir: Path::new("target/flatbuffers/"),
            ..Default::default()
        })
        .expect("flatc generated rust file(s)");
}
