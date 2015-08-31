#![feature(custom_derive, plugin, test)]
#![plugin(serde_macros)]

extern crate bincode;
extern crate capnp;
extern crate rmp_serialize as msgpack;
extern crate protobuf;
extern crate num;
extern crate serde;
extern crate serde_json;
extern crate rustc_serialize;
extern crate test;

pub mod goser;

/*
#[cfg(test)]
pub mod writer;
*/

// Unfortunately these need to be at the toplevel of the module.
pub mod country_capnp {
    include!(concat!(env!("OUT_DIR"), "/country_capnp.rs"));
}

pub mod log_capnp {
    include!(concat!(env!("OUT_DIR"), "/log_capnp.rs"));
}

pub mod log_proto;
