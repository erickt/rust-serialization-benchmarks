This is a suite of serialization benchmarks. In comprises of the following projects:

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
% git submodule sync
% make
```
