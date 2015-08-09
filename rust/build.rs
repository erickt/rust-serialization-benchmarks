extern crate capnpc;

fn main() {
    ::capnpc::compile("src", &["src/country.capnp", "src/log.capnp"]).unwrap();
}
