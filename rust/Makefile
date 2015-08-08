all: proto
	cargo build

.PHONY: test
test: proto
	cargo test

.PHONY: bench
bench: proto
	cargo bench

proto: src/log_proto.rs

src/log_proto.rs: src/log_proto.proto
	protoc --rust_out src src/log_proto.proto
