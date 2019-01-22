extern crate capnpc;
extern crate flatc_rust;
extern crate protoc_rust;

use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/log.capnp");
    capnpc::CompilerCommand::new()
        .src_prefix("src")
        .file("src/log.capnp")
        .run()
        .expect("schema compiler command");

    println!("cargo:rerun-if-changed=src/log_proto.proto");
    let out_dir = "target/protobufs/";
    std::fs::create_dir_all(out_dir).expect("could not create protobufs output folder");
    protoc_rust::run(protoc_rust::Args {
        input: &["src/log_proto.proto"],
        out_dir,
        ..Default::default()
    }).expect("protoc");

    println!("cargo:rerun-if-changed=src/log.fbs");
    flatc_rust::run(flatc_rust::Args {
        inputs: &[Path::new("src/log.fbs")],
        out_dir: Path::new("target/flatbuffers/"),
        ..Default::default()
    }).expect("flatc");
}
