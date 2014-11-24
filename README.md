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

To run the tests, do:

```
% git submodule init
% git submodule update
% make
```

---

Current results:

```
test goser::bincode::bench_decoder                          ... bench:      7682 ns/iter (+/- 3680) = 52 MB/s
test goser::bincode::bench_encoder                          ... bench:       516 ns/iter (+/- 265) = 775 MB/s
test goser::bincode::bench_populate                         ... bench:      1504 ns/iter (+/- 324)
test goser::capnp::bench_deserialize                        ... bench:       251 ns/iter (+/- 140) = 1784 MB/s
test goser::capnp::bench_deserialize_packed_unbuffered      ... bench:      1344 ns/iter (+/- 533) = 250 MB/s
test goser::capnp::bench_populate                           ... bench:       663 ns/iter (+/- 236)
test goser::capnp::bench_serialize                          ... bench:       144 ns/iter (+/- 37) = 3111 MB/s
test goser::capnp::bench_serialize_packed_unbuffered        ... bench:       913 ns/iter (+/- 436) = 369 MB/s
test goser::msgpack::bench_decoder                          ... bench:      3411 ns/iter (+/- 1837) = 84 MB/s
test goser::msgpack::bench_encoder                          ... bench:       961 ns/iter (+/- 477) = 298 MB/s
test goser::msgpack::bench_populate                         ... bench:      1564 ns/iter (+/- 453)
test goser::protobuf::bench_decoder                         ... bench:      3116 ns/iter (+/- 1485) = 91 MB/s
test goser::protobuf::bench_encoder                         ... bench:      1220 ns/iter (+/- 482) = 234 MB/s
test goser::protobuf::bench_populate                        ... bench:       942 ns/iter (+/- 836)
test goser::serde_json::bench_deserializer                  ... bench:     13372 ns/iter (+/- 7310) = 45 MB/s
test goser::serde_json::bench_populate                      ... bench:      1497 ns/iter (+/- 450)
test goser::serde_json::bench_serializer                    ... bench:      4233 ns/iter (+/- 1090) = 142 MB/s
test goser::serialize_json::bench_decoder                   ... bench:     31934 ns/iter (+/- 16186) = 18 MB/s
test goser::serialize_json::bench_encoder                   ... bench:      8481 ns/iter (+/- 3392) = 71 MB/s
test goser::serialize_json::bench_populate                  ... bench:      1471 ns/iter (+/- 426)
```
