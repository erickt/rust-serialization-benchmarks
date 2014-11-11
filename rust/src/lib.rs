#![feature(phase, macro_rules)]

extern crate capnp;
extern crate msgpack;
extern crate serde;
extern crate serialize;
extern crate test;

#[phase(plugin)]
extern crate serde_macros;

#[cfg(test)]
pub mod goser;

// Unfortunately these need to be at the toplevel of the module.
mod country_capnp;
mod log_capnp;
