#![feature(alloc, core, io, plugin, slicing_syntax, test)]
#![plugin(serde_macros)]

//extern crate serialize;
extern crate alloc;
extern crate bincode;
extern crate capnp;
extern crate msgpack;
extern crate protobuf;
extern crate serde;
extern crate "rustc-serialize" as rustc_serialize;
extern crate test;

pub mod goser;

#[cfg(test)]
pub mod writer;

// Unfortunately these need to be at the toplevel of the module.
#[allow(dead_code)]
pub mod country_capnp;

#[allow(dead_code)]
pub mod log_capnp;

pub mod log_proto;
