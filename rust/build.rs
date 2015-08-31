extern crate capnpc;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let prefix = Path::new("src/");

    if let Some(path) = env::var_os("PATH") {
        let mut paths = env::split_paths(&path).collect::<Vec<_>>();
        paths.push(PathBuf::from("../go/goser/capnproto-c++-0.5.2"));
        let new_path = env::join_paths(paths.iter()).unwrap();
        env::set_var("PATH", &new_path);
    }

    ::capnpc::compile(
      &prefix,
      &[
        &prefix.join("country.capnp"),
        &prefix.join("log.capnp"),
      ],
    ).unwrap();
}
