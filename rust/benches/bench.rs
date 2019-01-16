#![feature(test)]
extern crate bincode;
extern crate capnp;
extern crate flatbuffers;
extern crate protobuf;
extern crate rmp_serde;
extern crate rust_serde_benchmarks;
extern crate serde;
extern crate serde_json;
extern crate test;

use capnp::message::{Builder, ReaderOptions};
use protobuf::{Clear, Message};
use rust_serde_benchmarks::{log_capnp, log_proto, log_flatbuffers, protocol_capnp, protocol_protobuf, protocol_flatbuffers, Log};
use serde::{Deserialize, Serialize};
use test::Bencher;

#[bench]
fn clone(b: &mut Bencher) {
    let log = Log::new();

    let mut size = ::std::mem::size_of_val(&log);

    size += log.http.content_type.len();
    size += log.http.user_agent.len();
    size += log.http.referer.len();
    size += log.http.request_uri.len();

    size += log.origin.ip.len();
    size += log.origin.hostname.len();

    size += log.server_ip.len();
    size += log.server_name.len();
    size += log.remote_ip.len();
    size += log.ray_id.len();

    b.bytes = size as u64;

    b.iter(|| log.clone());
}

#[bench]
fn serde_json_serialize(b: &mut Bencher) {
    let log = Log::new();
    let mut buf = Vec::new();

    serde_json::to_writer(&mut buf, &log).unwrap();
    b.bytes = buf.len() as u64;

    b.iter(|| {
        buf.clear();
        serde_json::to_writer(&mut buf, &log).unwrap();
    });
}

#[bench]
fn serde_json_deserialize(b: &mut Bencher) {
    let log = Log::new();
    let json = serde_json::to_string(&log).unwrap();

    b.bytes = json.len() as u64;

    b.iter(|| serde_json::from_str::<Log>(&json).unwrap());
}

#[bench]
fn capnp_populate(b: &mut Bencher) {
    let mut msg = Builder::new_default();

    b.iter(|| {
        protocol_capnp::populate_log(&mut msg);
    });
}

#[bench]
fn capnp_serialize(b: &mut Bencher) {
    let mut msg = Builder::new_default();
    protocol_capnp::populate_log(&mut msg);

    let mut buf = Vec::new();
    ::capnp::serialize::write_message(&mut buf, &msg).unwrap();
    b.bytes = buf.len() as u64;

    b.iter(|| {
        buf.clear();
        ::capnp::serialize::write_message(&mut buf, &msg).unwrap();
    });
}

#[bench]
fn capnp_serialize_packed(b: &mut Bencher) {
    let mut msg = Builder::new_default();
    protocol_capnp::populate_log(&mut msg);

    let mut buf = Vec::new();
    ::capnp::serialize_packed::write_message(&mut buf, &msg).unwrap();
    b.bytes = buf.len() as u64;

    b.iter(|| {
        buf.clear();
        ::capnp::serialize_packed::write_message(&mut buf, &msg).unwrap();
    });
}

#[bench]
fn capnp_deserialize(b: &mut Bencher) {
    let mut msg = Builder::new_default();
    protocol_capnp::populate_log(&mut msg);

    let mut buf = Vec::new();
    ::capnp::serialize::write_message(&mut buf, &msg).unwrap();
    b.bytes = buf.len() as u64;

    b.iter(|| {
        let mut rdr = &*buf;
        let message_reader =
            ::capnp::serialize::read_message(&mut rdr, ReaderOptions::new()).unwrap();
        message_reader.get_root::<log_capnp::log::Reader>().unwrap();
    });
}

#[bench]
fn capnp_deserialize_packed(b: &mut Bencher) {
    let mut msg = Builder::new_default();
    protocol_capnp::populate_log(&mut msg);

    let mut buf = Vec::new();
    ::capnp::serialize_packed::write_message(&mut buf, &msg).unwrap();
    b.bytes = buf.len() as u64;

    b.iter(|| {
        let mut rdr = &*buf;
        let message_reader =
            ::capnp::serialize_packed::read_message(&mut rdr, ReaderOptions::new()).unwrap();
        message_reader.get_root::<log_capnp::log::Reader>().unwrap();
    });
}

#[bench]
fn flatbuffers_populate_with_args(b: &mut Bencher) {
    let mut msg = flatbuffers::FlatBufferBuilder::new();

    b.iter(|| {
        msg.reset();
        let log = protocol_flatbuffers::populate_log_with_args(&mut msg);
        msg.finish(log, None);
    });
}

#[bench]
fn flatbuffers_populate_with_builder(b: &mut Bencher) {
    let mut msg = flatbuffers::FlatBufferBuilder::new();

    b.iter(|| {
        msg.reset();
        let log = protocol_flatbuffers::populate_log_with_builder(&mut msg);
        msg.finish(log, None);
    });
}

#[bench]
fn flatbuffers_serialize(b: &mut Bencher) {
    let mut msg = flatbuffers::FlatBufferBuilder::new();
    let log = protocol_flatbuffers::populate_log_with_builder(&mut msg);
    msg.finish(log, None);

    let buf = msg.finished_data();
    b.bytes = buf.len() as u64;

    b.iter(|| {
        msg.finished_data()
    });
}

#[bench]
fn flatbuffers_deserialize(b: &mut Bencher) {
    let mut msg = flatbuffers::FlatBufferBuilder::new();
    let log = protocol_flatbuffers::populate_log_with_builder(&mut msg);
    msg.finish(log, None);

    let buf = msg.finished_data();
    b.bytes = buf.len() as u64;

    b.iter(|| {
        log_flatbuffers::get_root_as_log(&buf)
    });
}

#[bench]
fn rmp_serde_serialize(b: &mut Bencher) {
    let mut buf = Vec::new();
    let log = Log::new();

    log.serialize(&mut ::rmp_serde::Serializer::new(&mut buf))
        .unwrap();
    b.bytes = buf.len() as u64;

    b.iter(|| {
        buf.clear();
        log.serialize(&mut ::rmp_serde::Serializer::new(&mut buf))
            .unwrap();
    });
}

#[bench]
fn rmp_serde_deserialize(b: &mut Bencher) {
    let mut buf = Vec::new();
    let log = Log::new();
    log.serialize(&mut ::rmp_serde::Serializer::new(&mut buf))
        .unwrap();
    b.bytes = buf.len() as u64;

    b.iter(|| {
        let mut decoder = ::rmp_serde::Deserializer::new(&*buf);
        let _log: Log = Deserialize::deserialize(&mut decoder).unwrap();
    });
}

#[bench]
fn rust_protobuf_populate(b: &mut Bencher) {
    b.iter(|| protocol_protobuf::new_log());
}

#[bench]
fn rust_protobuf_serialize(b: &mut Bencher) {
    let mut buf = Vec::new();
    let log = protocol_protobuf::new_log();

    log.write_to_writer(&mut buf).unwrap();
    b.bytes = buf.len() as u64;

    b.iter(|| {
        buf.clear();
        log.write_to_writer(&mut buf).unwrap();
    });
}

#[bench]
fn rust_protobuf_deserialize(b: &mut Bencher) {
    let log = protocol_protobuf::new_log();
    let buf = log.write_to_bytes().unwrap();
    b.bytes = buf.len() as u64;

    let mut log = log_proto::Log::new();
    b.iter(|| {
        log.clear();
        log.merge_from_bytes(&buf).unwrap();
    });
}

#[bench]
fn rust_bincode_serialize(b: &mut Bencher) {
    let mut buf = Vec::new();
    let log = Log::new();

    ::bincode::serialize_into(&mut buf, &log).unwrap();
    b.bytes = buf.len() as u64;

    b.iter(|| {
        buf.clear();
        ::bincode::serialize_into(&mut buf, &log).unwrap();
    });
}

#[bench]
fn rust_bincode_deserialize(b: &mut Bencher) {
    let buf = bincode::serialize(&Log::new()).unwrap();
    b.bytes = buf.len() as u64;

    b.iter(|| {
        let _log: Log = ::bincode::deserialize_from(&*buf).unwrap();
    });
}
