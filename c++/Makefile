all: log
	./log

log: log.cc bench.h
	c++ -O3 -o log -I rapidjson/include log.cc

clean:
	rm -f log
