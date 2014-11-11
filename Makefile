all: c++ go rust

.PHONY: c++ go rust

c++:
	cd c++ && $(MAKE) bench

go:
	cd go && $(MAKE) bench

rust:
	cd rust && cargo bench
