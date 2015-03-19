extern crate capnpc;

use std::path::Path;

fn main() {
    let prefix = Path::new("src/");

    ::capnpc::compile(
      &prefix,
      &[
        &prefix.join("country.capnp"),
        &prefix.join("log.capnp"),
      ],
    ).unwrap();
}
