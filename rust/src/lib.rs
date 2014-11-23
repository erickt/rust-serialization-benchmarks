#![feature(phase, macro_rules, slicing_syntax)]

extern crate bincode;
extern crate capnp;
extern crate msgpack;
extern crate protobuf;
extern crate serde;
extern crate serialize;
extern crate test;

#[phase(plugin)]
extern crate serde_macros;

pub mod goser;
//pub mod writer;

// Unfortunately these need to be at the toplevel of the module.
#[allow(dead_code)]
pub mod country_capnp;

#[allow(dead_code)]
pub mod log_capnp;

pub mod log_proto;
