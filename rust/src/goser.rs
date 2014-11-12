use serialize;
use serde::ser;
use serde::de;

#[deriving(Encodable, Decodable)]
#[deriving_serializable]
#[deriving_deserializable]
pub struct Http {
    protocol: HttpProtocol,
    status: u32,
    host_status: u32,
    up_status: u32,
    method: HttpMethod,
    content_type: String,
    user_agent: String,
    referer: String,
    request_uri: String,
}

#[allow(non_camel_case_types)]
#[deriving(Show, FromPrimitive)]
pub enum HttpProtocol {
    HTTP_PROTOCOL_UNKNOWN,
    HTTP10,
    HTTP11,
}

impl<S: serialize::Encoder<E>, E> serialize::Encodable<S, E> for HttpProtocol {
    fn encode(&self, s: &mut S) -> Result<(), E> {
        (*self as uint).encode(s)
    }
}

impl<D: ::serialize::Decoder<E>, E> serialize::Decodable<D, E> for HttpProtocol {
    fn decode(d: &mut D) -> Result<HttpProtocol, E> {
        match FromPrimitive::from_uint(try!(d.read_uint())) {
            Some(value) => Ok(value),
            None => Err(d.error("cannot convert from uint")),
        }
    }
}

impl<S: ser::Serializer<E>, E> ser::Serializable<S, E> for HttpProtocol {
    #[inline]
    fn serialize(&self, s: &mut S) -> Result<(), E> {
        s.serialize_uint(*self as uint)
    }
}

impl<D: de::Deserializer<E>, E> de::Deserializable<D, E> for HttpProtocol {
    #[inline]
    fn deserialize_token(d: &mut D, token: de::Token) -> Result<HttpProtocol, E> {
        d.expect_from_primitive(token)
    }
}

#[allow(non_camel_case_types)]
#[deriving(Show, FromPrimitive)]
pub enum HttpMethod {
    METHOD_UNKNOWN,
    GET,
    POST,
    DELETE,
    PUT,
    HEAD,
    PURGE,
    OPTIONS,
    PROPFIND,
    MKCOL,
    PATCH,
}

impl<S: serialize::Encoder<E>, E> serialize::Encodable<S, E> for HttpMethod {
    fn encode(&self, s: &mut S) -> Result<(), E> {
        (*self as uint).encode(s)
    }
}

impl<D: ::serialize::Decoder<E>, E> serialize::Decodable<D, E> for HttpMethod {
    fn decode(d: &mut D) -> Result<HttpMethod, E> {
        match FromPrimitive::from_uint(try!(d.read_uint())) {
            Some(value) => Ok(value),
            None => Err(d.error("cannot convert from uint")),
        }
    }
}

impl<S: ser::Serializer<E>, E> ser::Serializable<S, E> for HttpMethod {
    #[inline]
    fn serialize(&self, s: &mut S) -> Result<(), E> {
        s.serialize_uint(*self as uint)
    }
}

impl<D: de::Deserializer<E>, E> de::Deserializable<D, E> for HttpMethod {
    #[inline]
    fn deserialize_token(d: &mut D, token: de::Token) -> Result<HttpMethod, E> {
        d.expect_from_primitive(token)
    }
}

#[allow(non_camel_case_types)]
#[deriving(Show, FromPrimitive)]
pub enum CacheStatus {
    CACHESTATUS_UNKNOWN,
    Miss,
    Expired,
    Hit,
}

impl<S: serialize::Encoder<E>, E> serialize::Encodable<S, E> for CacheStatus {
    fn encode(&self, s: &mut S) -> Result<(), E> {
        (*self as uint).encode(s)
    }
}

impl<D: ::serialize::Decoder<E>, E> serialize::Decodable<D, E> for CacheStatus {
    fn decode(d: &mut D) -> Result<CacheStatus, E> {
        match FromPrimitive::from_uint(try!(d.read_uint())) {
            Some(value) => Ok(value),
            None => Err(d.error("cannot convert from uint")),
        }
    }
}

impl<S: ser::Serializer<E>, E> ser::Serializable<S, E> for CacheStatus {
    #[inline]
    fn serialize(&self, s: &mut S) -> Result<(), E> {
        s.serialize_uint(*self as uint)
    }
}

impl<D: de::Deserializer<E>, E> de::Deserializable<D, E> for CacheStatus {
    #[inline]
    fn deserialize_token(d: &mut D, token: de::Token) -> Result<CacheStatus, E> {
        d.expect_from_primitive(token)
    }
}

#[deriving(Encodable, Decodable)]
#[deriving_serializable]
#[deriving_deserializable]
pub struct Origin {
    ip: String,
    port: u32,
    hostname: String,
    protocol: OriginProtocol,
}

#[allow(non_camel_case_types)]
#[deriving(Show, FromPrimitive)]
pub enum OriginProtocol {
    ORIGIN_PROTOCOL_UNKNOWN,
    HTTP,
    HTTPS,
}

impl<S: serialize::Encoder<E>, E> serialize::Encodable<S, E> for OriginProtocol {
    fn encode(&self, s: &mut S) -> Result<(), E> {
        (*self as uint).encode(s)
    }
}

impl<D: ::serialize::Decoder<E>, E> serialize::Decodable<D, E> for OriginProtocol {
    fn decode(d: &mut D) -> Result<OriginProtocol, E> {
        match FromPrimitive::from_uint(try!(d.read_uint())) {
            Some(value) => Ok(value),
            None => Err(d.error("cannot convert from uint")),
        }
    }
}

impl<S: ser::Serializer<E>, E> ser::Serializable<S, E> for OriginProtocol {
    #[inline]
    fn serialize(&self, s: &mut S) -> Result<(), E> {
        s.serialize_uint(*self as uint)
    }
}

impl<D: de::Deserializer<E>, E> de::Deserializable<D, E> for OriginProtocol {
    #[inline]
    fn deserialize_token(d: &mut D, token: de::Token) -> Result<OriginProtocol, E> {
        d.expect_from_primitive(token)
    }
}

#[allow(non_camel_case_types)]
#[deriving(Show, FromPrimitive)]
pub enum ZonePlan {
    ZONEPLAN_UNKNOWN,
    FREE,
    PRO,
    BIZ,
    ENT,
}

impl<S: serialize::Encoder<E>, E> serialize::Encodable<S, E> for ZonePlan {
    fn encode(&self, s: &mut S) -> Result<(), E> {
        (*self as uint).encode(s)
    }
}

impl<D: ::serialize::Decoder<E>, E> serialize::Decodable<D, E> for ZonePlan {
    fn decode(d: &mut D) -> Result<ZonePlan, E> {
        match FromPrimitive::from_uint(try!(d.read_uint())) {
            Some(value) => Ok(value),
            None => Err(d.error("cannot convert from uint")),
        }
    }
}

impl<S: ser::Serializer<E>, E> ser::Serializable<S, E> for ZonePlan {
    #[inline]
    fn serialize(&self, s: &mut S) -> Result<(), E> {
        s.serialize_uint(*self as uint)
    }
}

impl<D: de::Deserializer<E>, E> de::Deserializable<D, E> for ZonePlan {
    #[inline]
    fn deserialize_token(d: &mut D, token: de::Token) -> Result<ZonePlan, E> {
        d.expect_from_primitive(token)
    }
}

#[deriving(Show, FromPrimitive)]
pub enum Country {
	UNKNOWN,
	A1,
	A2,
	O1,
	AD,
	AE,
	AF,
	AG,
	AI,
	AL,
	AM,
	AO,
	AP,
	AQ,
	AR,
	AS,
	AT,
	AU,
	AW,
	AX,
	AZ,
	BA,
	BB,
	BD,
	BE,
	BF,
	BG,
	BH,
	BI,
	BJ,
	BL,
	BM,
	BN,
	BO,
	BQ,
	BR,
	BS,
	BT,
	BV,
	BW,
	BY,
	BZ,
	CA,
	CC,
	CD,
	CF,
	CG,
	CH,
	CI,
	CK,
	CL,
	CM,
	CN,
	CO,
	CR,
	CU,
	CV,
	CW,
	CX,
	CY,
	CZ,
	DE,
	DJ,
	DK,
	DM,
	DO,
	DZ,
	EC,
	EE,
	EG,
	EH,
	ER,
	ES,
	ET,
	EU,
	FI,
	FJ,
	FK,
	FM,
	FO,
	FR,
	GA,
	GB,
	GD,
	GE,
	GF,
	GG,
	GH,
	GI,
	GL,
	GM,
	GN,
	GP,
	GQ,
	GR,
	GS,
	GT,
	GU,
	GW,
	GY,
	HK,
	HM,
	HN,
	HR,
	HT,
	HU,
	ID,
	IE,
	IL,
	IM,
	IN,
	IO,
	IQ,
	IR,
	IS,
	IT,
	JE,
	JM,
	JO,
	JP,
	KE,
	KG,
	KH,
	KI,
	KM,
	KN,
	KP,
	KR,
	KW,
	KY,
	KZ,
	LA,
	LB,
	LC,
	LI,
	LK,
	LR,
	LS,
	LT,
	LU,
	LV,
	LY,
	MA,
	MC,
	MD,
	ME,
	MF,
	MG,
	MH,
	MK,
	ML,
	MM,
	MN,
	MO,
	MP,
	MQ,
	MR,
	MS,
	MT,
	MU,
	MV,
	MW,
	MX,
	MY,
	MZ,
	NA,
	NC,
	NE,
	NF,
	NG,
	NI,
	NL,
	NO,
	NP,
	NR,
	NU,
	NZ,
	OM,
	PA,
	PE,
	PF,
	PG,
	PH,
	PK,
	PL,
	PM,
	PN,
	PR,
	PS,
	PT,
	PW,
	PY,
	QA,
	RE,
	RO,
	RS,
	RU,
	RW,
	SA,
	SB,
	SC,
	SD,
	SE,
	SG,
	SH,
	SI,
	SJ,
	SK,
	SL,
	SM,
	SN,
	SO,
	SR,
	SS,
	ST,
	SV,
	SX,
	SY,
	SZ,
	TC,
	TD,
	TF,
	TG,
	TH,
	TJ,
	TK,
	TL,
	TM,
	TN,
	TO,
	TR,
	TT,
	TV,
	TW,
	TZ,
	UA,
	UG,
	UM,
	US,
	UY,
	UZ,
	VA,
	VC,
	VE,
	VG,
	VI,
	VN,
	VU,
	WF,
	WS,
	XX,
	YE,
	YT,
	ZA,
	ZM,
	ZW,
}

impl<S: serialize::Encoder<E>, E> serialize::Encodable<S, E> for Country {
    fn encode(&self, s: &mut S) -> Result<(), E> {
        (*self as uint).encode(s)
    }
}

impl<D: ::serialize::Decoder<E>, E> serialize::Decodable<D, E> for Country {
    fn decode(d: &mut D) -> Result<Country, E> {
        match FromPrimitive::from_uint(try!(d.read_uint())) {
            Some(value) => Ok(value),
            None => Err(d.error("cannot convert from uint")),
        }
    }
}

impl<S: ser::Serializer<E>, E> ser::Serializable<S, E> for Country {
    #[inline]
    fn serialize(&self, s: &mut S) -> Result<(), E> {
        s.serialize_uint(*self as uint)
    }
}

impl<D: de::Deserializer<E>, E> de::Deserializable<D, E> for Country {
    #[inline]
    fn deserialize_token(d: &mut D, token: de::Token) -> Result<Country, E> {
        d.expect_from_primitive(token)
    }
}

#[deriving(Encodable, Decodable)]
#[deriving_serializable]
#[deriving_deserializable]
pub struct Log {
    timestamp: i64,
    zone_id: u32,
    zone_plan: ZonePlan,
    http: Http,
    origin: Origin,
    country: Country,
    cache_status: CacheStatus,
    server_ip: String,
    server_name: String,
    remote_ip: String,
    bytes_dlv: u64,
    ray_id: String,
}

impl Log {
    pub fn new() -> Log {
        Log {
            timestamp: 2837513946597,
            zone_id: 123456,
            zone_plan: FREE,
            http: Http {
                protocol: HTTP11,
                status: 200,
                host_status: 503,
                up_status: 520,
                method: GET,
                content_type: "text/html".to_string(),
                user_agent: "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/33.0.1750.146 Safari/537.36".to_string(),
                referer: "https://www.cloudflare.com/".to_string(),
                request_uri: "/cdn-cgi/trace".to_string(),
            },
            origin: Origin {
                ip: "1.2.3.4".to_string(),
                port: 8000,
                hostname: "www.example.com".to_string(),
                protocol: HTTPS,
            },
            country: US,
            cache_status: Hit,
            server_ip: "192.168.1.1".to_string(),
            server_name: "metal.cloudflare.com".to_string(),
            remote_ip: "10.1.2.3".to_string(),
            bytes_dlv: 123456,
            ray_id: "10c73629cce30078-LAX".to_string(),
        }
    }
}

#[cfg(test)]
mod capnp {
    use std::io::BufReader;
    use test::Bencher;

    use capnp;
    use capnp::message::MallocMessageBuilder;
    use capnp::ReaderOptions;
    use capnp::message::{MessageReader, MessageBuilder};

    use log_capnp;
    use country_capnp;

    fn new_log<'a, M: MessageBuilder<'a>>(msg: &'a mut M) -> log_capnp::log::Builder<'a> {
        let log = msg.init_root::<log_capnp::log::Builder>();
        log.set_timestamp(2837513946597);
        log.set_zone_id(123456);
        log.set_zone_plan(log_capnp::zone_plan::Free);

        let http = log.init_http();
        http.set_protocol(log_capnp::h_t_t_p::protocol::Http11);
        http.set_host_status(200);
        http.set_up_status(520);
        http.set_method(log_capnp::h_t_t_p::method::Get);
        http.set_content_type("text/html");
        http.set_user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/33.0.1750.146 Safari/537.36");
        http.set_referer("https://www.cloudflare.com/");
        http.set_request_u_r_i("/cdn-cgi/trace");

        let origin = log.init_origin();
        origin.set_ip("1.2.3.4");
        origin.set_port(8000);
        origin.set_hostname("www.example.com");
        origin.set_protocol(log_capnp::origin::protocol::Https);

        log.set_country(country_capnp::country::Us);
        log.set_cache_status(log_capnp::cache_status::Hit);
        log.set_server_ip("192.168.1.1");
        log.set_server_name("metal.cloudflare.com");
        log.set_remote_ip("10.1.2.3");
        log.set_bytes_dlv(123456);
        log.set_ray_id("10c73629cce30078-LAX");

        log
    }

    #[bench]
    fn bench_populate(b: &mut Bencher) {
        let mut msg = MallocMessageBuilder::new_default();

        b.iter(|| {
            new_log(&mut msg);
        });
    }

    #[bench]
    fn bench_serialize(b: &mut Bencher) {
        let mut msg = MallocMessageBuilder::new_default();
        new_log(&mut msg);

        let mut bytes = Vec::new();
        capnp::serialize::write_message(&mut bytes, &msg).unwrap();
        b.bytes = bytes.len() as u64;

        b.iter(|| {
            bytes.clear();
            capnp::serialize::write_message(&mut bytes, &msg).unwrap();
        });
    }

    #[bench]
    fn bench_deserialize(b: &mut Bencher) {
        let mut msg = MallocMessageBuilder::new_default();
        new_log(&mut msg);

        let mut bytes = Vec::new();
        capnp::serialize::write_message(&mut bytes, &msg).unwrap();

        b.bytes = bytes.len() as u64;

        b.iter(|| {
            let mut rdr = BufReader::new(bytes.as_slice());
            let message_reader = capnp::serialize::new_reader(&mut rdr, ReaderOptions::new()).unwrap();
            let _: log_capnp::log::Reader = message_reader.get_root::<log_capnp::log::Reader>();
        });
    }

    #[bench]
    fn bench_serialize_packed_unbuffered(b: &mut Bencher) {
        let mut msg = MallocMessageBuilder::new_default();
        new_log(&mut msg);
    
        let mut bytes = Vec::new();
        capnp::serialize_packed::write_packed_message_unbuffered(&mut bytes, &msg).unwrap();
        b.bytes = bytes.len() as u64;

        b.iter(|| {
            bytes.clear();
            capnp::serialize_packed::write_packed_message_unbuffered(&mut bytes, &msg).unwrap();
        });
    }

    #[bench]
    fn bench_deserialize_packed_unbuffered(b: &mut Bencher) {
        let mut msg = MallocMessageBuilder::new_default();
        new_log(&mut msg);
    
        let mut bytes = Vec::new();
        capnp::serialize_packed::write_packed_message_unbuffered(&mut bytes, &msg).unwrap();

        b.bytes = bytes.len() as u64;

        b.iter(|| {
            let mut rdr = BufReader::new(bytes.as_slice());
            let message_reader = capnp::serialize_packed::new_reader_unbuffered(
                &mut rdr,
                ReaderOptions::new()
            ).unwrap();
            let _: log_capnp::log::Reader = message_reader.get_root::<log_capnp::log::Reader>();
        });
    }
}

#[cfg(test)]
mod serialize_json {
    use test::Bencher;
    use serialize::json;
    use serialize::{Encodable, Decodable};

    use super::Log;

    fn write_to_bytes(bytes: &mut Vec<u8>, log: &Log) {
        let mut encoder = json::Encoder::new(bytes as &mut Writer);
        log.encode(&mut encoder).unwrap()
    }

    #[bench]
    fn bench_populate(b: &mut Bencher) {
        b.iter(|| {
            Log::new();
        });
    }

    #[bench]
    fn bench_encoder(b: &mut Bencher) {
        let log = Log::new();
        let mut bytes = Vec::new();
        write_to_bytes(&mut bytes, &log);
        b.bytes = bytes.len() as u64;

        b.iter(|| {
            bytes.clear();
            write_to_bytes(&mut bytes, &log);
        });
    }

    #[bench]
    fn bench_decoder(b: &mut Bencher) {
        let log = Log::new();
        let json = json::encode(&log);
        b.bytes = json.len() as u64;

        b.iter(|| {
            let json = json::from_str(json.as_slice()).unwrap();
            let mut decoder = json::Decoder::new(json);
            let _log: Log = Decodable::decode(&mut decoder).unwrap();
        });
    }
}

#[cfg(test)]
mod serde_json {
    use std::io::AsRefWriter;
    use test::Bencher;

    use serde::Serializable;
    use serde::json;

    use super::Log;

    fn write_to_bytes(bytes: &mut Vec<u8>, log: &Log) {
        let mut serializer = json::Serializer::new(bytes.by_ref());
        log.serialize(&mut serializer).unwrap();
    }

    #[bench]
    fn bench_populate(b: &mut Bencher) {
        b.iter(|| {
            Log::new();
        });
    }

    #[bench]
    fn bench_serializer(b: &mut Bencher) {
        let log = Log::new();
        let mut bytes = Vec::new();
        write_to_bytes(&mut bytes, &log);
        b.bytes = bytes.len() as u64;

        b.iter(|| {
            bytes.clear();
            write_to_bytes(&mut bytes, &log);
        });
    }

    #[bench]
    fn bench_deserializer(b: &mut Bencher) {
        let log = Log::new();
        let json = json::to_string(&log).unwrap();
        b.bytes = json.len() as u64;

        b.iter(|| {
            let _log: Log = json::from_str(json.as_slice()).unwrap();
        });
    }
}

#[cfg(test)]
mod msgpack {
    use test::Bencher;
    use serialize::Encodable;

    use msgpack;

    use super::Log;

    fn write_to_bytes(bytes: &mut Vec<u8>, log: &Log) {
        let mut encoder = msgpack::Encoder::new(bytes);
        log.encode(&mut encoder).unwrap()
    }

    #[bench]
    fn bench_populate(b: &mut Bencher) {
        b.iter(|| {
            Log::new();
        });
    }

    #[bench]
    fn bench_encoder(b: &mut Bencher) {
        let log = Log::new();
        let mut bytes = Vec::new();
        write_to_bytes(&mut bytes, &log);
        b.bytes = bytes.len() as u64;

        b.iter(|| {
            bytes.clear();
            write_to_bytes(&mut bytes, &log);
        });
    }

    #[bench]
    fn bench_decoder(b: &mut Bencher) {
        let mut bytes = Vec::new();
        write_to_bytes(&mut bytes, &Log::new());
        b.bytes = bytes.len() as u64;

        b.iter(|| {
            let _log: Log = msgpack::from_msgpack(bytes.as_slice()).unwrap();
        });
    }
}

#[cfg(test)]
mod protobuf {
    use test::Bencher;

    use protobuf;
    use protobuf::Message;
    use log_proto;

    fn new_log() -> log_proto::Log {
        let mut log = log_proto::Log::new();
        log.set_timestamp(2837513946597);
        log.set_zone_id(123456);
        log.set_zone_plan(log_proto::FREE);

        let mut http = log_proto::HTTP::new();
        http.set_protocol(log_proto::HTTP_HTTP11);
        http.set_host_status(200);
        http.set_up_status(520);
        http.set_method(log_proto::HTTP_GET);
        http.set_content_type("text/html".to_string());
        http.set_user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/33.0.1750.146 Safari/537.36".to_string());
        http.set_referer("https://www.cloudflare.com/".to_string());
        http.set_request_uri("/cdn-cgi/trace".to_string());
        log.set_http(http);

        let mut origin = log_proto::Origin::new();
        origin.set_ip([1, 2, 3, 4].to_vec());
        origin.set_port(8000);
        origin.set_hostname("www.example.com".to_string());
        origin.set_protocol(log_proto::Origin_HTTPS);
        log.set_origin(origin);

        log.set_country(log_proto::US);
        log.set_cache_status(log_proto::HIT);
        log.set_server_ip([192, 168, 1, 1].to_vec());
        log.set_server_name("metal.cloudflare.com".to_string());
        log.set_remote_ip([10, 1, 2, 3].to_vec());
        log.set_bytes_dlv(123456);
        log.set_ray_id("10c73629cce30078-LAX".to_string());

        log
    }

    #[bench]
    fn bench_populate(b: &mut Bencher) {
        b.iter(|| {
            new_log();
        });
    }

    #[bench]
    fn bench_encoder(b: &mut Bencher) {
        let log = new_log();
        let mut bytes = Vec::new();
        log.write_to_writer(&mut bytes).unwrap();
        b.bytes = bytes.len() as u64;

        b.iter(|| {
            bytes.clear();
            log.write_to_writer(&mut bytes).unwrap();
        });
    }

    #[bench]
    fn bench_decoder(b: &mut Bencher) {
        let log = new_log();
        let bytes = log.write_to_bytes().unwrap();
        b.bytes = bytes.len() as u64;

        b.iter(|| {
            let _log: log_proto::Log = protobuf::parse_from_bytes(bytes.as_slice()).unwrap();
        });
    }
}

#[cfg(test)]
mod bincode {
    use std::io::BufReader;
    use test::Bencher;

    use bincode;

    use super::Log;

    #[bench]
    fn bench_populate(b: &mut Bencher) {
        b.iter(|| {
            Log::new();
        });
    }

    #[bench]
    fn bench_encoder(b: &mut Bencher) {
        let log = Log::new();
        let mut bytes = Vec::new();
        bincode::encode_into(&log, &mut bytes).unwrap();
        b.bytes = bytes.len() as u64;

        b.iter(|| {
            bytes.clear();
            bincode::encode_into(&log, &mut bytes).unwrap();
        });
    }

    #[bench]
    fn bench_decoder(b: &mut Bencher) {
        let log = Log::new();
        let mut bytes = Vec::new();
        bincode::encode_into(&log, &mut bytes).unwrap();
        b.bytes = bytes.len() as u64;

        b.iter(|| {
            let mut rdr = BufReader::new(bytes.as_slice());
            let _log: Log = bincode::decode_from(&mut rdr).unwrap();
        });
    }
}
