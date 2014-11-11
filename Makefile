all: rust

.PHONY: c++
c++:
	cd c++ && $(MAKE) bench

.PHONY: rust
rust:
	cd rust && cargo bench


