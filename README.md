This is a suite of serialization benchmarks. It comprises of the following projects:

* C++
  * [rapidjson](https://github.com/erickt/rapidjson)
* Go
  * [encoding/json](http://golang.org/pkg/encoding/json)
  * [ffjson](https://github.com/pquerna/ffjson)
  * [go-capnproto](https://github.com/glycerine/go-capnproto)
  * [gogoprotobuf](http://code.google.com/p/gogoprotobuf/)
  * [goprotobuf](http://code.google.com/p/goprotobuf/)
* Rust
  * [bincode](https://github.com/TyOverby/bincode)
  * [capnproto-rust](https://github.com/dwrensha/capnproto-rust)
  * [rust-msgpack](https://github.com/mneumann/rust-msgpack)
  * [rust-protobuf](https://github.com/stepancheg/rust-protobuf)
  * [rust-serde](https://github.com/erickt/rust-serde)
  * [serialize](http://doc.rust-lang.org/serialize/)
  * [flatbuffers](https://github.com/google/flatbuffers)

The initial benchmark suite is derived from
[Goser](https://github.com/cloudflare/goser) benchmark, which is all about
serializing and serializing from this JSON record:

```json
{
    "timestamp": 25469139677502,
    "zone_id": 123456,
    "zone_plan": 1,
    "http": {
        "protocol": 2,
        "status": 200,
        "host_status": 503,
        "up_status": 520,
        "method": 1,
        "content_type": "text/html",
        "user_agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML,like Gecko) Chrome/33.0.1750.146 Safari/537.36",
        "referer": "https: //www.cloudflare.com/",
        "request_uri": "/cdn-cgi/trace"
    },
    "origin": {
        "ip": "1.2.3.4",
        "port": 8000,
        "hostname": "www.example.com",
        "protocol": 2
    },
    "country": 238,
    "cache_status": 3,
    "server_ip": "192.168.1.1",
    "server_name": "metal.cloudflare.com",
    "remote_ip": "10.1.2.3",
    "bytes_dlv": 123456,
    "ray_id": "10c73629cce30078-LAX"
}
```

To run the tests, you'll need to install:

* [Cap'n Proto](https://capnproto.org/)
* [Protocol Buffers](https://developers.google.com/protocol-buffers/?hl=en)
* [Go](https://golang.org/)
* [Rust](https://rust-lang.org/)

On OS X and [Homebrew](http://brew.sh/), this can all be done with:

```
brew install capnp protobuf rust go
```

I'm not yet sure what's needed for other operating systems. Once that is
installed, run:

```
% git submodule init
% git submodule update
% make
```

---

Current results:

RapidJSON:

```
log populate       :  549 ns/iter
log serialize      : 1930 ns/iter = 310 MB/s
log dom deserialize: 3972 ns/iter = 151 MB/s
log sax deserialize: 3821 ns/iter = 157 MB/s
str serialize      : 2642 ns/iter = 388 MB/s
str dom deserialize: 3006 ns/iter = 341 MB/s
str sax deserialize: 3536 ns/iter = 290 MB/s
```

Go:

```
BenchmarkPopulatePb              10000000  1235 ns/op     400 B/op   25 allocs/op
BenchmarkPopulateGogopb          50000000   331 ns/op      48 B/op    3 allocs/op
BenchmarkPopulateCapnp           10000000  2225 ns/op     112 B/op    2 allocs/op
BenchmarkMarshalJSON              2000000  6707 ns/op   89.16 MB/s  568 B/op       29 allocs/op
BenchmarkMarshalPb               10000000  1237 ns/op  233.52 MB/s    0 B/op        0 allocs/op
BenchmarkMarshalGogopb           30000000   448 ns/op  644.71 MB/s  320 B/op        1 allocs/op
BenchmarkMarshalCapnp           100000000   114 ns/op 4039.83 MB/s    8 B/op        1 allocs/op
BenchmarkUnmarshalJSON             500000 24924 ns/op   23.99 MB/s 2144 B/op       41 allocs/op
BenchmarkUnmarshalPb              5000000  3486 ns/op   82.89 MB/s  858 B/op       23 allocs/op
BenchmarkUnmarshalGogopb         20000000  1040 ns/op  277.68 MB/s  266 B/op       10 allocs/op
BenchmarkUnmarshalCapnp          20000000   613 ns/op  756.17 MB/s  256 B/op        5 allocs/op
BenchmarkUnmarshalCapnpZeroCopy  50000000   321 ns/op 1445.07 MB/s   88 B/op        3 allocs/op
```

Rust:

```
test goser::bench_clone                          ... bench:         333 ns/iter (+/- 129) = 1645 MB/s
test goser::bincode::bench_decoder               ... bench:       1,399 ns/iter (+/- 571) = 285 MB/s
test goser::bincode::bench_encoder               ... bench:         135 ns/iter (+/- 43) = 2962 MB/s
test goser::bincode::bench_populate              ... bench:         878 ns/iter (+/- 116)
test goser::bincode_serde::bench_deserialize     ... bench:       1,188 ns/iter (+/- 460) = 301 MB/s
test goser::bincode_serde::bench_populate        ... bench:         900 ns/iter (+/- 312)
test goser::bincode_serde::bench_serialize       ... bench:         170 ns/iter (+/- 46) = 2105 MB/s
test goser::capnp::bench_deserialize             ... bench:         344 ns/iter (+/- 56) = 1302 MB/s
test goser::capnp::bench_deserialize_packed      ... bench:         812 ns/iter (+/- 360) = 415 MB/s
test goser::capnp::bench_populate                ... bench:         644 ns/iter (+/- 344)
test goser::capnp::bench_serialize               ... bench:          32 ns/iter (+/- 19) = 14000 MB/s
test goser::capnp::bench_serialize_packed        ... bench:         564 ns/iter (+/- 307) = 597 MB/s
test goser::msgpack::bench_decoder               ... bench:       2,234 ns/iter (+/- 831) = 128 MB/s
test goser::msgpack::bench_deserializer          ... bench:       2,686 ns/iter (+/- 1,117) = 106 MB/s
test goser::msgpack::bench_encoder               ... bench:         784 ns/iter (+/- 471) = 366 MB/s
test goser::msgpack::bench_populate              ... bench:       1,063 ns/iter (+/- 471)
test goser::msgpack::bench_serializer            ... bench:         922 ns/iter (+/- 183) = 311 MB/s
test goser::protobuf::bench_decoder              ... bench:       2,016 ns/iter (+/- 554) = 141 MB/s
test goser::protobuf::bench_encoder              ... bench:         779 ns/iter (+/- 444) = 367 MB/s
test goser::protobuf::bench_populate             ... bench:         908 ns/iter (+/- 264)
test goser::rustc_serialize_json::bench_decoder  ... bench:      30,541 ns/iter (+/- 7,753) = 19 MB/s
test goser::rustc_serialize_json::bench_encoder  ... bench:       3,469 ns/iter (+/- 1,583) = 174 MB/s
test goser::rustc_serialize_json::bench_populate ... bench:       1,010 ns/iter (+/- 400)
test goser::serde_json::bench_deserializer       ... bench:       4,726 ns/iter (+/- 2,393) = 128 MB/s
test goser::serde_json::bench_populate           ... bench:         949 ns/iter (+/- 216)
test goser::serde_json::bench_serializer         ... bench:       1,966 ns/iter (+/- 692) = 307 MB/s
```
