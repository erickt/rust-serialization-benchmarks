all: capn proto
	cargo build

.PHONY: test
test: capn proto
	cargo test

.PHONY: bench
bench: capn proto
	cargo test

capn: src/country_capnp.rs src/log_capnp.rs

src/country_capnp.rs: src/country.capnp
	capnpc -o rust src/country.capnp

src/log_capnp.rs: src/log.capnp
	capnpc -o rust src/log.capnp

proto: src/log_proto.rs

src/log_proto.rs: src/log_proto.proto
	protoc --rust_out src src/log_proto.proto
