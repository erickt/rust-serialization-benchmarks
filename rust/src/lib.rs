extern crate bincode;
extern crate capnp;
extern crate flatbuffers;
extern crate num_traits;
#[macro_use]
extern crate num_derive;
extern crate protobuf;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::borrow::Cow;
use std::fmt;

pub mod log_capnp {
    include!(concat!(env!("OUT_DIR"), "/log_capnp.rs"));
}

#[path = "../target/protobufs/log_proto.rs"]
pub mod log_proto;

#[allow(non_snake_case)]
#[path = "../target/flatbuffers/log_generated.rs"]
pub mod log_flatbuffers;

macro_rules! enum_number {
    ($name:ident { $($variant:ident = $value:expr, )* }) => {
        #[derive(Clone, Copy, Debug, FromPrimitive)]
        pub enum $name {
            $($variant = $value,)*
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.serialize_u64(*self as u64)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("positive integer")
                    }

                    fn visit_u64<E>(self, value: u64) -> Result<$name, E>
                    where
                        E: ::serde::de::Error,
                    {
                        match value {
                            $( $value => Ok($name::$variant), )*
                            _ => Err(E::custom(
                                format!("unknown {} value: {}",
                                stringify!($name), value))),
                        }
                    }
                }

                deserializer.deserialize_u64(Visitor)
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Http<'a> {
    pub protocol: HttpProtocol,
    pub status: u32,
    pub host_status: u32,
    pub up_status: u32,
    pub method: HttpMethod,
    #[serde(borrow)]
    pub content_type: Cow<'a, str>,
    #[serde(borrow)]
    pub user_agent: Cow<'a, str>,
    #[serde(borrow)]
    pub referer: Cow<'a, str>,
    #[serde(borrow)]
    pub request_uri: Cow<'a, str>,
}

enum_number!(HttpProtocol {
    HttpProtocolUnknown = 0,
    HTTP10 = 1,
    HTTP11 = 2,
});

enum_number!(HttpMethod {
    MethodUnknown = 0,
    GET = 1,
    POST = 2,
    DELETE = 3,
    PUT = 4,
    HEAD = 5,
    PURGE = 6,
    OPTIONS = 7,
    PROPFIND = 8,
    MKCOL = 9,
    PATCH = 10,
});

enum_number!(CacheStatus {
    CacheStatusUnknown = 0,
    Miss = 1,
    Expired = 2,
    Hit = 3,
});

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Origin<'a> {
    #[serde(borrow)]
    pub ip: Cow<'a, str>,
    pub port: u32,
    #[serde(borrow)]
    pub hostname: Cow<'a, str>,
    pub protocol: OriginProtocol,
}

enum_number!(OriginProtocol {
    OriginProtocolUnknown = 0,
    HTTP = 1,
    HTTPS = 2,
});

enum_number!(ZonePlan {
    ZonePlanUnknown = 0,
    FREE = 1,
    PRO = 2,
    BIZ = 3,
    ENT = 4,
});

enum_number!(Country {
    UNKNOWN = 0,
    A1 = 1,
    A2 = 2,
    O1 = 3,
    AD = 4,
    AE = 5,
    AF = 6,
    AG = 7,
    AI = 8,
    AL = 9,
    AM = 10,
    AO = 11,
    AP = 12,
    AQ = 13,
    AR = 14,
    AS = 15,
    AT = 16,
    AU = 17,
    AW = 18,
    AX = 19,
    AZ = 20,
    BA = 21,
    BB = 22,
    BD = 23,
    BE = 24,
    BF = 25,
    BG = 26,
    BH = 27,
    BI = 28,
    BJ = 29,
    BL = 30,
    BM = 31,
    BN = 32,
    BO = 33,
    BQ = 34,
    BR = 35,
    BS = 36,
    BT = 37,
    BV = 38,
    BW = 39,
    BY = 40,
    BZ = 41,
    CA = 42,
    CC = 43,
    CD = 44,
    CF = 45,
    CG = 46,
    CH = 47,
    CI = 48,
    CK = 49,
    CL = 50,
    CM = 51,
    CN = 52,
    CO = 53,
    CR = 54,
    CU = 55,
    CV = 56,
    CW = 57,
    CX = 58,
    CY = 59,
    CZ = 60,
    DE = 61,
    DJ = 62,
    DK = 63,
    DM = 64,
    DO = 65,
    DZ = 66,
    EC = 67,
    EE = 68,
    EG = 69,
    EH = 70,
    ER = 71,
    ES = 72,
    ET = 73,
    EU = 74,
    FI = 75,
    FJ = 76,
    FK = 77,
    FM = 78,
    FO = 79,
    FR = 80,
    GA = 81,
    GB = 82,
    GD = 83,
    GE = 84,
    GF = 85,
    GG = 86,
    GH = 87,
    GI = 88,
    GL = 89,
    GM = 90,
    GN = 91,
    GP = 92,
    GQ = 93,
    GR = 94,
    GS = 95,
    GT = 96,
    GU = 97,
    GW = 98,
    GY = 99,
    HK = 100,
    HM = 101,
    HN = 102,
    HR = 103,
    HT = 104,
    HU = 105,
    ID = 106,
    IE = 107,
    IL = 108,
    IM = 109,
    IN = 110,
    IO = 111,
    IQ = 112,
    IR = 113,
    IS = 114,
    IT = 115,
    JE = 116,
    JM = 117,
    JO = 118,
    JP = 119,
    KE = 120,
    KG = 121,
    KH = 122,
    KI = 123,
    KM = 124,
    KN = 125,
    KP = 126,
    KR = 127,
    KW = 128,
    KY = 129,
    KZ = 130,
    LA = 131,
    LB = 132,
    LC = 133,
    LI = 134,
    LK = 135,
    LR = 136,
    LS = 137,
    LT = 138,
    LU = 139,
    LV = 140,
    LY = 141,
    MA = 142,
    MC = 143,
    MD = 144,
    ME = 145,
    MF = 146,
    MG = 147,
    MH = 148,
    MK = 149,
    ML = 150,
    MM = 151,
    MN = 152,
    MO = 153,
    MP = 154,
    MQ = 155,
    MR = 156,
    MS = 157,
    MT = 158,
    MU = 159,
    MV = 160,
    MW = 161,
    MX = 162,
    MY = 163,
    MZ = 164,
    NA = 165,
    NC = 166,
    NE = 167,
    NF = 168,
    NG = 169,
    NI = 170,
    NL = 171,
    NO = 172,
    NP = 173,
    NR = 174,
    NU = 175,
    NZ = 176,
    OM = 177,
    PA = 178,
    PE = 179,
    PF = 180,
    PG = 181,
    PH = 182,
    PK = 183,
    PL = 184,
    PM = 185,
    PN = 186,
    PR = 187,
    PS = 188,
    PT = 189,
    PW = 190,
    PY = 191,
    QA = 192,
    RE = 193,
    RO = 194,
    RS = 195,
    RU = 196,
    RW = 197,
    SA = 198,
    SB = 199,
    SC = 200,
    SD = 201,
    SE = 202,
    SG = 203,
    SH = 204,
    SI = 205,
    SJ = 206,
    SK = 207,
    SL = 208,
    SM = 209,
    SN = 210,
    SO = 211,
    SR = 212,
    SS = 213,
    ST = 214,
    SV = 215,
    SX = 216,
    SY = 217,
    SZ = 218,
    TC = 219,
    TD = 220,
    TF = 221,
    TG = 222,
    TH = 223,
    TJ = 224,
    TK = 225,
    TL = 226,
    TM = 227,
    TN = 228,
    TO = 229,
    TR = 230,
    TT = 231,
    TV = 232,
    TW = 233,
    TZ = 234,
    UA = 235,
    UG = 236,
    UM = 237,
    US = 238,
    UY = 239,
    UZ = 240,
    VA = 241,
    VC = 242,
    VE = 243,
    VG = 244,
    VI = 245,
    VN = 246,
    VU = 247,
    WF = 248,
    WS = 249,
    XX = 250,
    YE = 251,
    YT = 252,
    ZA = 253,
    ZM = 254,
    ZW = 255,
});

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Log<'a> {
    pub timestamp: i64,
    pub zone_id: u32,
    pub zone_plan: ZonePlan,
    #[serde(borrow)]
    pub http: Http<'a>,
    #[serde(borrow)]
    pub origin: Origin<'a>,
    pub country: Country,
    pub cache_status: CacheStatus,
    #[serde(borrow)]
    pub server_ip: Cow<'a, str>,
    #[serde(borrow)]
    pub server_name: Cow<'a, str>,
    #[serde(borrow)]
    pub remote_ip: Cow<'a, str>,
    pub bytes_dlv: u64,
    #[serde(borrow)]
    pub ray_id: Cow<'a, str>,
}

impl<'a> Log<'a> {
    pub fn new() -> Log<'a> {
        Log {
            timestamp: 2837513946597,
            zone_id: 123456,
            zone_plan: ZonePlan::FREE,
            http: Http {
                protocol: HttpProtocol::HTTP11,
                status: 200,
                host_status: 503,
                up_status: 520,
                method: HttpMethod::GET,
                content_type: "text/html".into(),
                user_agent: "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/33.0.1750.146 Safari/537.36".into(),
                referer: "https://www.cloudflare.com/".into(),
                request_uri: "/cdn-cgi/trace".into(),
            },
            origin: Origin {
                ip: "1.2.3.4".into(),
                port: 8000,
                hostname: "www.example.com".into(),
                protocol: OriginProtocol::HTTPS,
            },
            country: Country::US,
            cache_status: CacheStatus::Hit,
            server_ip: "192.168.1.1".into(),
            server_name: "metal.cloudflare.com".into(),
            remote_ip: "10.1.2.3".into(),
            bytes_dlv: 123456,
            ray_id: "10c73629cce30078-LAX".into(),
        }
    }
}

pub mod protocol_capnp {
    use crate::log_capnp::{h_t_t_p, log, origin, CacheStatus, Country, ZonePlan};
    use capnp::message::{Allocator, Builder};

    pub fn populate_log<'a, A: Allocator>(msg: &'a mut Builder<A>) -> log::Builder<'a> {
        let mut log = msg.init_root::<log::Builder>();
        log.set_timestamp(2837513946597);
        log.set_zone_id(123456);
        log.set_zone_plan(ZonePlan::Free);

        {
            let mut http = log.reborrow().init_http();
            http.set_protocol(h_t_t_p::Protocol::Http11);
            http.set_host_status(200);
            http.set_up_status(520);
            http.set_method(h_t_t_p::Method::Get);
            http.set_content_type("text/html");
            http.set_user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/33.0.1750.146 Safari/537.36");
            http.set_referer("https://www.cloudflare.com/");
            http.set_request_u_r_i("/cdn-cgi/trace");
        }

        {
            let mut origin = log.reborrow().init_origin();
            origin.set_ip("1.2.3.4");
            origin.set_port(8000);
            origin.set_hostname("www.example.com");
            origin.set_protocol(origin::Protocol::Https);
        }

        log.set_country(Country::Us);
        log.set_cache_status(CacheStatus::Hit);
        log.set_server_ip("192.168.1.1");
        log.set_server_name("metal.cloudflare.com");
        log.set_remote_ip("10.1.2.3");
        log.set_bytes_dlv(123456);
        log.set_ray_id("10c73629cce30078-LAX");

        log
    }
}

pub mod protocol_protobuf {
    use crate::log_proto;

    pub fn new_log() -> log_proto::Log {
        let mut log = log_proto::Log::new();
        log.set_timestamp(2837513946597);
        log.set_zone_id(123456);
        log.set_zone_plan(log_proto::ZonePlan::FREE);

        let mut http = log_proto::HTTP::new();
        http.set_protocol(log_proto::HTTP_Protocol::HTTP11);
        http.set_host_status(200);
        http.set_up_status(520);
        http.set_method(log_proto::HTTP_Method::GET);
        http.set_content_type("text/html".to_string());
        http.set_user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/33.0.1750.146 Safari/537.36".to_string());
        http.set_referer("https://www.cloudflare.com/".to_string());
        http.set_request_uri("/cdn-cgi/trace".to_string());
        log.set_http(http);

        let mut origin = log_proto::Origin::new();
        origin.set_ip([1, 2, 3, 4].to_vec());
        origin.set_port(8000);
        origin.set_hostname("www.example.com".to_string());
        origin.set_protocol(log_proto::Origin_Protocol::HTTPS);
        log.set_origin(origin);

        log.set_country(log_proto::Country::US);
        log.set_cache_status(log_proto::CacheStatus::HIT);
        log.set_server_ip([192, 168, 1, 1].to_vec());
        log.set_server_name("metal.cloudflare.com".to_string());
        log.set_remote_ip([10, 1, 2, 3].to_vec());
        log.set_bytes_dlv(123456);
        log.set_ray_id("10c73629cce30078-LAX".to_string());

        log
    }
}

pub mod protocol_flatbuffers {
    use crate::log_flatbuffers;

    pub fn populate_log_with_args<'a, 'b>(
        msg: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> flatbuffers::WIPOffset<log_flatbuffers::Log<'a>> {
        let content_type = msg.create_string("text/html");
        let user_agent = msg.create_string("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/33.0.1750.146 Safari/537.36");
        let referer = msg.create_string("https://www.cloudflare.com/");
        let request_uri = msg.create_string("/cdn-cgi/trace");

        let http = log_flatbuffers::HTTP::create(
            msg,
            &log_flatbuffers::HTTPArgs {
                protocol: log_flatbuffers::HTTP_Protocol::http11,
                hostStatus: 200,
                upStatus: 520,
                method: log_flatbuffers::HTTP_Method::get,
                contentType: Some(content_type),
                userAgent: Some(user_agent),
                referer: Some(referer),
                requestURI: Some(request_uri),
                ..Default::default()
            },
        );

        let ip = Some(msg.create_string("1.2.3.4"));
        let hostname = Some(msg.create_string("www.example.com"));

        let origin = log_flatbuffers::Origin::create(
            msg,
            &log_flatbuffers::OriginArgs {
                ip,
                port: 8000,
                hostname,
                protocol: log_flatbuffers::Origin_Protocol::https,
                ..Default::default()
            },
        );

        let server_ip = msg.create_string("192.168.1.1");
        let server_name = msg.create_string("metal.cloudflare.com");
        let remote_ip = msg.create_string("10.1.2.3");
        let ray_id = msg.create_string("10c73629cce30078-LAX");

        log_flatbuffers::Log::create(
            msg,
            &log_flatbuffers::LogArgs {
                http: Some(http),
                origin: Some(origin),
                timestamp: 2837513946597,
                zoneId: 123456,
                zonePlan: log_flatbuffers::ZonePlan::free,
                country: log_flatbuffers::Country::us,
                cacheStatus: log_flatbuffers::CacheStatus::hit,
                serverIp: Some(server_ip),
                serverName: Some(server_name),
                remoteIp: Some(remote_ip),
                bytesDlv: 123456,
                rayId: Some(ray_id),
                ..Default::default()
            },
        )
    }

    pub fn populate_log_with_builder<'a, 'b>(
        msg: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> flatbuffers::WIPOffset<log_flatbuffers::Log<'a>> {
        let content_type = msg.create_string("text/html");
        let user_agent = msg.create_string("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/33.0.1750.146 Safari/537.36");
        let referer = msg.create_string("https://www.cloudflare.com/");
        let request_uri = msg.create_string("/cdn-cgi/trace");

        let http = {
            let mut http_builder = log_flatbuffers::HTTPBuilder::new(msg);
            http_builder.add_protocol(log_flatbuffers::HTTP_Protocol::http11);
            http_builder.add_hostStatus(200);
            http_builder.add_upStatus(520);
            http_builder.add_method(log_flatbuffers::HTTP_Method::get);
            http_builder.add_contentType(content_type);
            http_builder.add_userAgent(user_agent);
            http_builder.add_referer(referer);
            http_builder.add_requestURI(request_uri);
            http_builder.finish()
        };

        let ip = msg.create_string("1.2.3.4");
        let hostname = msg.create_string("www.example.com");

        let origin = {
            let mut origin_builder = log_flatbuffers::OriginBuilder::new(msg);
            origin_builder.add_ip(ip);
            origin_builder.add_port(8000);
            origin_builder.add_hostname(hostname);
            origin_builder.add_protocol(log_flatbuffers::Origin_Protocol::https);
            origin_builder.finish()
        };

        let server_ip = msg.create_string("192.168.1.1");
        let server_name = msg.create_string("metal.cloudflare.com");
        let remote_ip = msg.create_string("10.1.2.3");
        let ray_id = msg.create_string("10c73629cce30078-LAX");

        let mut log_builder = log_flatbuffers::LogBuilder::new(msg);
        log_builder.add_http(http);
        log_builder.add_origin(origin);
        log_builder.add_timestamp(2837513946597);
        log_builder.add_zoneId(123456);
        log_builder.add_zonePlan(log_flatbuffers::ZonePlan::free);
        log_builder.add_country(log_flatbuffers::Country::us);
        log_builder.add_cacheStatus(log_flatbuffers::CacheStatus::hit);
        log_builder.add_serverIp(server_ip);
        log_builder.add_serverName(server_name);
        log_builder.add_remoteIp(remote_ip);
        log_builder.add_bytesDlv(123456);
        log_builder.add_rayId(ray_id);
        log_builder.finish()
    }
}
