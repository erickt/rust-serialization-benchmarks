extern crate capnpc;
extern crate protoc_rust;

fn main() {
    println!("cargo:rerun-if-changed=src/log.capnp");
    capnpc::CompilerCommand::new()
        .src_prefix("src")
        .file("src/log.capnp")
        .run()
        .expect("schema compiler command");

    println!("cargo:rerun-if-changed=src/log_proto.proto");
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/",
        input: &["src/log_proto.proto"],
        ..Default::default()
    }).expect("protoc");
}
