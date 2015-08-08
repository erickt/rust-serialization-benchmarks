use std::str::FromStr;

use num::FromPrimitive;
use rustc_serialize;
use serde::de;
use serde::ser;

#[derive(RustcEncodable, RustcDecodable)]
#[derive(Serialize)]
#[derive(Deserialize)]
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
#[derive(Copy, Clone, Debug)]
pub enum HttpProtocol {
    HTTP_PROTOCOL_UNKNOWN,
    HTTP10,
    HTTP11,
}

impl FromStr for HttpProtocol {
    type Err = ();
    fn from_str(_s: &str) -> Result<HttpProtocol, ()> { unimplemented!() }
}

impl FromPrimitive for HttpProtocol {
    fn from_i64(i: i64) -> Option<HttpProtocol> {
        FromPrimitive::from_u64(i as u64)
    }

    fn from_u64(n: u64) -> Option<HttpProtocol> {
        match n {
            0 => Some(HttpProtocol::HTTP_PROTOCOL_UNKNOWN),
            1 => Some(HttpProtocol::HTTP10),
            2 => Some(HttpProtocol::HTTP11),
            _ => None,
        }
    }
}

impl rustc_serialize::Encodable for HttpProtocol {
    fn encode<S: rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        (*self as usize).encode(s)
    }
}

impl rustc_serialize::Decodable for HttpProtocol {
    fn decode<D: rustc_serialize::Decoder>(d: &mut D) -> Result<HttpProtocol, D::Error> {
        match FromPrimitive::from_usize(try!(d.read_usize())) {
            Some(value) => Ok(value),
            None => Err(d.error("cannot convert from usize")),
        }
    }
}

impl ser::Serialize for HttpProtocol {
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: ser::Serializer,
    {
        serializer.visit_u8(*self as u8)
    }
}

impl de::Deserialize for HttpProtocol {
    #[inline]
    fn deserialize<
        D: de::Deserializer,
    >(state: &mut D) -> Result<HttpProtocol, D::Error> {
        state.visit_u8(de::impls::PrimitiveVisitor::new())
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
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

impl FromStr for HttpMethod {
    type Err = ();
    fn from_str(_s: &str) -> Result<HttpMethod, ()> { unimplemented!() }
}

impl FromPrimitive for HttpMethod {
    fn from_i64(i: i64) -> Option<HttpMethod> {
        FromPrimitive::from_u64(i as u64)
    }

    fn from_u64(n: u64) -> Option<HttpMethod> {
        match n {
            0 => Some(HttpMethod::METHOD_UNKNOWN),
            1 => Some(HttpMethod::GET),
            2 => Some(HttpMethod::POST),
            3 => Some(HttpMethod::DELETE),
            4 => Some(HttpMethod::PUT),
            5 => Some(HttpMethod::HEAD),
            6 => Some(HttpMethod::PURGE),
            7 => Some(HttpMethod::OPTIONS),
            8 => Some(HttpMethod::PROPFIND),
            9 => Some(HttpMethod::MKCOL),
            10 => Some(HttpMethod::PATCH),
            _ => None,
        }
    }
}

impl rustc_serialize::Encodable for HttpMethod {
    fn encode<S: rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        (*self as usize).encode(s)
    }
}

impl rustc_serialize::Decodable for HttpMethod {
    fn decode<D: rustc_serialize::Decoder>(d: &mut D) -> Result<HttpMethod, D::Error> {
        match FromPrimitive::from_usize(try!(d.read_usize())) {
            Some(value) => Ok(value),
            None => Err(d.error("cannot convert from usize")),
        }
    }
}

impl ser::Serialize for HttpMethod {
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: ser::Serializer,
    {
        serializer.visit_u8(*self as u8)
    }
}

impl de::Deserialize for HttpMethod {
    #[inline]
    fn deserialize<
        S: de::Deserializer,
    >(state: &mut S) -> Result<HttpMethod, S::Error> {
        state.visit_u8(de::impls::PrimitiveVisitor::new())
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum CacheStatus {
    CACHESTATUS_UNKNOWN,
    Miss,
    Expired,
    Hit,
}

impl FromStr for CacheStatus {
    type Err = ();
    fn from_str(_s: &str) -> Result<CacheStatus, ()> { unimplemented!() }
}

impl FromPrimitive for CacheStatus {
    fn from_i64(i: i64) -> Option<CacheStatus> {
        FromPrimitive::from_u64(i as u64)
    }

    fn from_u64(n: u64) -> Option<CacheStatus> {
        match n {
            0 => Some(CacheStatus::CACHESTATUS_UNKNOWN),
            1 => Some(CacheStatus::Miss),
            2 => Some(CacheStatus::Expired),
            3 => Some(CacheStatus::Hit),
            _ => None,
        }
    }
}

impl rustc_serialize::Encodable for CacheStatus {
    fn encode<S: rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        (*self as usize).encode(s)
    }
}

impl rustc_serialize::Decodable for CacheStatus {
    fn decode<D: rustc_serialize::Decoder>(d: &mut D) -> Result<CacheStatus, D::Error> {
        match FromPrimitive::from_usize(try!(d.read_usize())) {
            Some(value) => Ok(value),
            None => Err(d.error("cannot convert from usize")),
        }
    }
}

impl ser::Serialize for CacheStatus {
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: ser::Serializer,
    {
        serializer.visit_u8(*self as u8)
    }
}

impl de::Deserialize for CacheStatus {
    #[inline]
    fn deserialize<
        S: de::Deserializer,
    >(state: &mut S) -> Result<CacheStatus, S::Error> {
        state.visit_u8(de::impls::PrimitiveVisitor::new())
    }
}

#[derive(RustcEncodable, RustcDecodable)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Origin {
    ip: String,
    port: u32,
    hostname: String,
    protocol: OriginProtocol,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum OriginProtocol {
    ORIGIN_PROTOCOL_UNKNOWN,
    HTTP,
    HTTPS,
}

impl FromStr for OriginProtocol {
    type Err = ();
    fn from_str(_s: &str) -> Result<OriginProtocol, ()> { unimplemented!() }
}

impl FromPrimitive for OriginProtocol {
    fn from_i64(i: i64) -> Option<OriginProtocol> {
        FromPrimitive::from_u64(i as u64)
    }

    fn from_u64(n: u64) -> Option<OriginProtocol> {
        match n {
            0 => Some(OriginProtocol::ORIGIN_PROTOCOL_UNKNOWN),
            1 => Some(OriginProtocol::HTTP),
            2 => Some(OriginProtocol::HTTPS),
            _ => None,
        }
    }
}

impl rustc_serialize::Encodable for OriginProtocol {
    fn encode<S: rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        (*self as usize).encode(s)
    }
}

impl rustc_serialize::Decodable for OriginProtocol {
    fn decode<D: rustc_serialize::Decoder>(d: &mut D) -> Result<OriginProtocol, D::Error> {
        match FromPrimitive::from_usize(try!(d.read_usize())) {
            Some(value) => Ok(value),
            None => Err(d.error("cannot convert from usize")),
        }
    }
}

impl ser::Serialize for OriginProtocol {
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: ser::Serializer,
    {
        serializer.visit_u8(*self as u8)
    }
}

impl de::Deserialize for OriginProtocol {
    #[inline]
    fn deserialize<
        S: de::Deserializer,
    >(state: &mut S) -> Result<OriginProtocol, S::Error> {
        state.visit_u8(de::impls::PrimitiveVisitor::new())
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum ZonePlan {
    ZONEPLAN_UNKNOWN,
    FREE,
    PRO,
    BIZ,
    ENT,
}

impl FromStr for ZonePlan {
    type Err = ();
    fn from_str(_s: &str) -> Result<ZonePlan, ()> { unimplemented!() }
}

impl FromPrimitive for ZonePlan {
    fn from_i64(i: i64) -> Option<ZonePlan> {
        FromPrimitive::from_u64(i as u64)
    }

    fn from_u64(n: u64) -> Option<ZonePlan> {
        match n {
            0 => Some(ZonePlan::ZONEPLAN_UNKNOWN),
            1 => Some(ZonePlan::FREE),
            2 => Some(ZonePlan::PRO),
            3 => Some(ZonePlan::BIZ),
            4 => Some(ZonePlan::ENT),
            _ => None,
        }
    }
}

impl rustc_serialize::Encodable for ZonePlan {
    fn encode<S: rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        (*self as usize).encode(s)
    }
}

impl rustc_serialize::Decodable for ZonePlan {
    fn decode<D: rustc_serialize::Decoder>(d: &mut D) -> Result<ZonePlan, D::Error> {
        match FromPrimitive::from_usize(try!(d.read_usize())) {
            Some(value) => Ok(value),
            None => Err(d.error("cannot convert from usize")),
        }
    }
}

impl ser::Serialize for ZonePlan {
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: ser::Serializer,
    {
        serializer.visit_u8(*self as u8)
    }
}

impl de::Deserialize for ZonePlan {
    #[inline]
    fn deserialize<
        S: de::Deserializer,
    >(state: &mut S) -> Result<ZonePlan, S::Error> {
        state.visit_u8(de::impls::PrimitiveVisitor::new())
    }
}

#[derive(Copy, Clone, Debug)]
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

impl FromStr for Country {
    type Err = ();
    fn from_str(_s: &str) -> Result<Country, ()> { unimplemented!() }
}

impl FromPrimitive for Country {
    fn from_i64(i: i64) -> Option<Country> {
        FromPrimitive::from_u64(i as u64)
    }

    fn from_u64(n: u64) -> Option<Country> {
        match n {
            0 => Some(Country::UNKNOWN),
            1 => Some(Country::A1),
            2 => Some(Country::A2),
            3 => Some(Country::O1),
            4 => Some(Country::AD),
            5 => Some(Country::AE),
            6 => Some(Country::AF),
            7 => Some(Country::AG),
            8 => Some(Country::AI),
            9 => Some(Country::AL),
            10 => Some(Country::AM),
            11 => Some(Country::AO),
            12 => Some(Country::AP),
            13 => Some(Country::AQ),
            14 => Some(Country::AR),
            15 => Some(Country::AS),
            16 => Some(Country::AT),
            17 => Some(Country::AU),
            18 => Some(Country::AW),
            19 => Some(Country::AX),
            20 => Some(Country::AZ),
            21 => Some(Country::BA),
            22 => Some(Country::BB),
            23 => Some(Country::BD),
            24 => Some(Country::BE),
            25 => Some(Country::BF),
            26 => Some(Country::BG),
            27 => Some(Country::BH),
            28 => Some(Country::BI),
            29 => Some(Country::BJ),
            30 => Some(Country::BL),
            31 => Some(Country::BM),
            32 => Some(Country::BN),
            33 => Some(Country::BO),
            34 => Some(Country::BQ),
            35 => Some(Country::BR),
            36 => Some(Country::BS),
            37 => Some(Country::BT),
            38 => Some(Country::BV),
            39 => Some(Country::BW),
            40 => Some(Country::BY),
            41 => Some(Country::BZ),
            42 => Some(Country::CA),
            43 => Some(Country::CC),
            44 => Some(Country::CD),
            45 => Some(Country::CF),
            46 => Some(Country::CG),
            47 => Some(Country::CH),
            48 => Some(Country::CI),
            49 => Some(Country::CK),
            50 => Some(Country::CL),
            51 => Some(Country::CM),
            52 => Some(Country::CN),
            53 => Some(Country::CO),
            54 => Some(Country::CR),
            55 => Some(Country::CU),
            56 => Some(Country::CV),
            57 => Some(Country::CW),
            58 => Some(Country::CX),
            59 => Some(Country::CY),
            60 => Some(Country::CZ),
            61 => Some(Country::DE),
            62 => Some(Country::DJ),
            63 => Some(Country::DK),
            64 => Some(Country::DM),
            65 => Some(Country::DO),
            66 => Some(Country::DZ),
            67 => Some(Country::EC),
            68 => Some(Country::EE),
            69 => Some(Country::EG),
            70 => Some(Country::EH),
            71 => Some(Country::ER),
            72 => Some(Country::ES),
            73 => Some(Country::ET),
            74 => Some(Country::EU),
            75 => Some(Country::FI),
            76 => Some(Country::FJ),
            77 => Some(Country::FK),
            78 => Some(Country::FM),
            79 => Some(Country::FO),
            80 => Some(Country::FR),
            81 => Some(Country::GA),
            82 => Some(Country::GB),
            83 => Some(Country::GD),
            84 => Some(Country::GE),
            85 => Some(Country::GF),
            86 => Some(Country::GG),
            87 => Some(Country::GH),
            88 => Some(Country::GI),
            89 => Some(Country::GL),
            90 => Some(Country::GM),
            91 => Some(Country::GN),
            92 => Some(Country::GP),
            93 => Some(Country::GQ),
            94 => Some(Country::GR),
            95 => Some(Country::GS),
            96 => Some(Country::GT),
            97 => Some(Country::GU),
            98 => Some(Country::GW),
            99 => Some(Country::GY),
            100 => Some(Country::HK),
            101 => Some(Country::HM),
            102 => Some(Country::HN),
            103 => Some(Country::HR),
            104 => Some(Country::HT),
            105 => Some(Country::HU),
            106 => Some(Country::ID),
            107 => Some(Country::IE),
            108 => Some(Country::IL),
            109 => Some(Country::IM),
            110 => Some(Country::IN),
            111 => Some(Country::IO),
            112 => Some(Country::IQ),
            113 => Some(Country::IR),
            114 => Some(Country::IS),
            115 => Some(Country::IT),
            116 => Some(Country::JE),
            117 => Some(Country::JM),
            118 => Some(Country::JO),
            119 => Some(Country::JP),
            120 => Some(Country::KE),
            121 => Some(Country::KG),
            122 => Some(Country::KH),
            123 => Some(Country::KI),
            124 => Some(Country::KM),
            125 => Some(Country::KN),
            126 => Some(Country::KP),
            127 => Some(Country::KR),
            128 => Some(Country::KW),
            129 => Some(Country::KY),
            130 => Some(Country::KZ),
            131 => Some(Country::LA),
            132 => Some(Country::LB),
            133 => Some(Country::LC),
            134 => Some(Country::LI),
            135 => Some(Country::LK),
            136 => Some(Country::LR),
            137 => Some(Country::LS),
            138 => Some(Country::LT),
            139 => Some(Country::LU),
            140 => Some(Country::LV),
            141 => Some(Country::LY),
            142 => Some(Country::MA),
            143 => Some(Country::MC),
            144 => Some(Country::MD),
            145 => Some(Country::ME),
            146 => Some(Country::MF),
            147 => Some(Country::MG),
            148 => Some(Country::MH),
            149 => Some(Country::MK),
            150 => Some(Country::ML),
            151 => Some(Country::MM),
            152 => Some(Country::MN),
            153 => Some(Country::MO),
            154 => Some(Country::MP),
            155 => Some(Country::MQ),
            156 => Some(Country::MR),
            157 => Some(Country::MS),
            158 => Some(Country::MT),
            159 => Some(Country::MU),
            160 => Some(Country::MV),
            161 => Some(Country::MW),
            162 => Some(Country::MX),
            163 => Some(Country::MY),
            164 => Some(Country::MZ),
            165 => Some(Country::NA),
            166 => Some(Country::NC),
            167 => Some(Country::NE),
            168 => Some(Country::NF),
            169 => Some(Country::NG),
            170 => Some(Country::NI),
            171 => Some(Country::NL),
            172 => Some(Country::NO),
            173 => Some(Country::NP),
            174 => Some(Country::NR),
            175 => Some(Country::NU),
            176 => Some(Country::NZ),
            177 => Some(Country::OM),
            178 => Some(Country::PA),
            179 => Some(Country::PE),
            180 => Some(Country::PF),
            181 => Some(Country::PG),
            182 => Some(Country::PH),
            183 => Some(Country::PK),
            184 => Some(Country::PL),
            185 => Some(Country::PM),
            186 => Some(Country::PN),
            187 => Some(Country::PR),
            188 => Some(Country::PS),
            189 => Some(Country::PT),
            190 => Some(Country::PW),
            191 => Some(Country::PY),
            192 => Some(Country::QA),
            193 => Some(Country::RE),
            194 => Some(Country::RO),
            195 => Some(Country::RS),
            196 => Some(Country::RU),
            197 => Some(Country::RW),
            198 => Some(Country::SA),
            199 => Some(Country::SB),
            200 => Some(Country::SC),
            201 => Some(Country::SD),
            202 => Some(Country::SE),
            203 => Some(Country::SG),
            204 => Some(Country::SH),
            205 => Some(Country::SI),
            206 => Some(Country::SJ),
            207 => Some(Country::SK),
            208 => Some(Country::SL),
            209 => Some(Country::SM),
            210 => Some(Country::SN),
            211 => Some(Country::SO),
            212 => Some(Country::SR),
            213 => Some(Country::SS),
            214 => Some(Country::ST),
            215 => Some(Country::SV),
            216 => Some(Country::SX),
            217 => Some(Country::SY),
            218 => Some(Country::SZ),
            219 => Some(Country::TC),
            220 => Some(Country::TD),
            221 => Some(Country::TF),
            222 => Some(Country::TG),
            223 => Some(Country::TH),
            224 => Some(Country::TJ),
            225 => Some(Country::TK),
            226 => Some(Country::TL),
            227 => Some(Country::TM),
            228 => Some(Country::TN),
            229 => Some(Country::TO),
            230 => Some(Country::TR),
            231 => Some(Country::TT),
            232 => Some(Country::TV),
            233 => Some(Country::TW),
            234 => Some(Country::TZ),
            235 => Some(Country::UA),
            236 => Some(Country::UG),
            237 => Some(Country::UM),
            238 => Some(Country::US),
            239 => Some(Country::UY),
            240 => Some(Country::UZ),
            241 => Some(Country::VA),
            242 => Some(Country::VC),
            243 => Some(Country::VE),
            244 => Some(Country::VG),
            245 => Some(Country::VI),
            246 => Some(Country::VN),
            247 => Some(Country::VU),
            248 => Some(Country::WF),
            249 => Some(Country::WS),
            250 => Some(Country::XX),
            251 => Some(Country::YE),
            252 => Some(Country::YT),
            253 => Some(Country::ZA),
            254 => Some(Country::ZM),
            255 => Some(Country::ZW),
            _ => None,
        }
    }
}

impl rustc_serialize::Encodable for Country {
    fn encode<S: rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        (*self as usize).encode(s)
    }
}

impl rustc_serialize::Decodable for Country {
    fn decode<D: rustc_serialize::Decoder>(d: &mut D) -> Result<Country, D::Error> {
        match FromPrimitive::from_usize(try!(d.read_usize())) {
            Some(value) => Ok(value),
            None => Err(d.error("cannot convert from usize")),
        }
    }
}

impl ser::Serialize for Country {
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: ser::Serializer,
    {
        serializer.visit_u8(*self as u8)
    }
}

impl de::Deserialize for Country {
    #[inline]
    fn deserialize<
        S: de::Deserializer,
    >(state: &mut S) -> Result<Country, S::Error> {
        state.visit_u8(de::impls::PrimitiveVisitor::new())
    }
}


#[derive(RustcEncodable, RustcDecodable, Deserialize)]
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

    #[automatically_derived]
    impl ::serde::ser::Serialize for Log {
        fn serialize<__S>(&self, serializer: &mut __S)
         -> ::std::result::Result<(), __S::Error> where
         __S: ::serde::ser::Serializer {
            {
                struct Visitor<'__a> {
                    state: usize,
                    value: &'__a Log,
                    _structure_ty: ::std::marker::PhantomData<&'__a Log>,
                }
                impl <'__a> ::serde::ser::MapVisitor for Visitor<'__a> {
                    #[inline]
                    fn visit<S>(&mut self, serializer: &mut S)
                     -> ::std::result::Result<Option<()>, S::Error> where
                     S: ::serde::ser::Serializer {
                        match self.state {
                            0usize => {
                                self.state += 1;
                                Ok(Some(match serializer.visit_struct_elt("timestamp",
                                                                          &self.value.timestamp)
                                            {
                                            ::std::result::Result::Ok(val) =>
                                            val,
                                            ::std::result::Result::Err(err) =>
                                            {
                                                return ::std::result::Result::Err(::std::convert::From::from(err))
                                            }
                                        }))
                            }
                            1usize => {
                                self.state += 1;
                                Ok(Some(match serializer.visit_struct_elt("zone_id",
                                                                          &self.value.zone_id)
                                            {
                                            ::std::result::Result::Ok(val) =>
                                            val,
                                            ::std::result::Result::Err(err) =>
                                            {
                                                return ::std::result::Result::Err(::std::convert::From::from(err))
                                            }
                                        }))
                            }
                            2usize => {
                                self.state += 1;
                                Ok(Some(match serializer.visit_struct_elt("zone_plan",
                                                                          &self.value.zone_plan)
                                            {
                                            ::std::result::Result::Ok(val) =>
                                            val,
                                            ::std::result::Result::Err(err) =>
                                            {
                                                return ::std::result::Result::Err(::std::convert::From::from(err))
                                            }
                                        }))
                            }
                            3usize => {
                                self.state += 1;
                                Ok(Some(match serializer.visit_struct_elt("http",
                                                                          &self.value.http)
                                            {
                                            ::std::result::Result::Ok(val) =>
                                            val,
                                            ::std::result::Result::Err(err) =>
                                            {
                                                return ::std::result::Result::Err(::std::convert::From::from(err))
                                            }
                                        }))
                            }
                            4usize => {
                                self.state += 1;
                                Ok(Some(match serializer.visit_struct_elt("origin",
                                                                          &self.value.origin)
                                            {
                                            ::std::result::Result::Ok(val) =>
                                            val,
                                            ::std::result::Result::Err(err) =>
                                            {
                                                return ::std::result::Result::Err(::std::convert::From::from(err))
                                            }
                                        }))
                            }
                            5usize => {
                                self.state += 1;
                                Ok(Some(match serializer.visit_struct_elt("country",
                                                                          &self.value.country)
                                            {
                                            ::std::result::Result::Ok(val) =>
                                            val,
                                            ::std::result::Result::Err(err) =>
                                            {
                                                return ::std::result::Result::Err(::std::convert::From::from(err))
                                            }
                                        }))
                            }
                            6usize => {
                                self.state += 1;
                                Ok(Some(match serializer.visit_struct_elt("cache_status",
                                                                          &self.value.cache_status)
                                            {
                                            ::std::result::Result::Ok(val) =>
                                            val,
                                            ::std::result::Result::Err(err) =>
                                            {
                                                return ::std::result::Result::Err(::std::convert::From::from(err))
                                            }
                                        }))
                            }
                            7usize => {
                                self.state += 1;
                                Ok(Some(match serializer.visit_struct_elt("server_ip",
                                                                          &self.value.server_ip)
                                            {
                                            ::std::result::Result::Ok(val) =>
                                            val,
                                            ::std::result::Result::Err(err) =>
                                            {
                                                return ::std::result::Result::Err(::std::convert::From::from(err))
                                            }
                                        }))
                            }
                            8usize => {
                                self.state += 1;
                                Ok(Some(match serializer.visit_struct_elt("server_name",
                                                                          &self.value.server_name)
                                            {
                                            ::std::result::Result::Ok(val) =>
                                            val,
                                            ::std::result::Result::Err(err) =>
                                            {
                                                return ::std::result::Result::Err(::std::convert::From::from(err))
                                            }
                                        }))
                            }
                            9usize => {
                                self.state += 1;
                                Ok(Some(match serializer.visit_struct_elt("remote_ip",
                                                                          &self.value.remote_ip)
                                            {
                                            ::std::result::Result::Ok(val) =>
                                            val,
                                            ::std::result::Result::Err(err) =>
                                            {
                                                return ::std::result::Result::Err(::std::convert::From::from(err))
                                            }
                                        }))
                            }
                            10usize => {
                                self.state += 1;
                                Ok(Some(match serializer.visit_struct_elt("bytes_dlv",
                                                                          &self.value.bytes_dlv)
                                            {
                                            ::std::result::Result::Ok(val) =>
                                            val,
                                            ::std::result::Result::Err(err) =>
                                            {
                                                return ::std::result::Result::Err(::std::convert::From::from(err))
                                            }
                                        }))
                            }
                            11usize => {
                                self.state += 1;
                                Ok(Some(match serializer.visit_struct_elt("ray_id",
                                                                          &self.value.ray_id)
                                            {
                                            ::std::result::Result::Ok(val) =>
                                            val,
                                            ::std::result::Result::Err(err) =>
                                            {
                                                return ::std::result::Result::Err(::std::convert::From::from(err))
                                            }
                                        }))
                            }
                            _ => Ok(None),
                        }
                    }
                    #[inline]
                    fn len(&self) -> Option<usize> { Some(12usize) }
                }
                serializer.visit_struct("Log",
                                        Visitor{value: self,
                                                state: 0,
                                                _structure_ty:
                                                    ::std::marker::PhantomData::<&Log>,})
            }
        }
    }


impl Log {
    pub fn new() -> Log {
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
                content_type: "text/html".to_string(),
                user_agent: "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/33.0.1750.146 Safari/537.36".to_string(),
                referer: "https://www.cloudflare.com/".to_string(),
                request_uri: "/cdn-cgi/trace".to_string(),
            },
            origin: Origin {
                ip: "1.2.3.4".to_string(),
                port: 8000,
                hostname: "www.example.com".to_string(),
                protocol: OriginProtocol::HTTPS,
            },
            country: Country::US,
            cache_status: CacheStatus::Hit,
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
    use test::{self, Bencher};

    use capnp;
    use capnp::message::{Allocator, Builder, Reader, ReaderOptions};

    use log_capnp;
    use country_capnp;

    fn new_log<'a, A: Allocator>(msg: &'a mut Builder<A>) -> log_capnp::log::Builder<'a> {
        let mut log = msg.init_root::<log_capnp::log::Builder>();
        log.set_timestamp(2837513946597);
        log.set_zone_id(123456);
        log.set_zone_plan(log_capnp::ZonePlan::Free);

        {
            let mut http = log.borrow().init_http();
            http.set_protocol(log_capnp::h_t_t_p::Protocol::Http11);
            http.set_host_status(200);
            http.set_up_status(520);
            http.set_method(log_capnp::h_t_t_p::Method::Get);
            http.set_content_type("text/html");
            http.set_user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/33.0.1750.146 Safari/537.36");
            http.set_referer("https://www.cloudflare.com/");
            http.set_request_u_r_i("/cdn-cgi/trace");
        }

        {
            let mut origin = log.borrow().init_origin();
            origin.set_ip("1.2.3.4");
            origin.set_port(8000);
            origin.set_hostname("www.example.com");
            origin.set_protocol(log_capnp::origin::Protocol::Https);
        }

        log.set_country(country_capnp::Country::Us);
        log.set_cache_status(log_capnp::CacheStatus::Hit);
        log.set_server_ip("192.168.1.1");
        log.set_server_name("metal.cloudflare.com");
        log.set_remote_ip("10.1.2.3");
        log.set_bytes_dlv(123456);
        log.set_ray_id("10c73629cce30078-LAX");

        log
    }

    #[bench]
    fn bench_populate(b: &mut Bencher) {
        let mut msg = Builder::new_default();

        b.iter(|| {
            let log = new_log(&mut msg);
            test::black_box(log);
        });
    }

    #[bench]
    fn bench_serialize(b: &mut Bencher) {
        let mut msg = Builder::new_default();
        new_log(&mut msg);

        let mut bytes = Vec::new();
        capnp::serialize::write_message(&mut bytes, &msg).unwrap();
        b.bytes = bytes.len() as u64;

        b.iter(|| {
            bytes.clear();
            capnp::serialize::write_message(&mut bytes, &msg).unwrap();
            test::black_box(&bytes);
        });
    }

    #[bench]
    fn bench_deserialize(b: &mut Bencher) {
        let mut msg = Builder::new_default();
        new_log(&mut msg);

        let mut bytes = Vec::new();
        capnp::serialize::write_message(&mut bytes, &msg).unwrap();

        b.bytes = bytes.len() as u64;

        b.iter(|| {
            let mut rdr = &*bytes;
            let message_reader = capnp::serialize::read_message(&mut rdr, ReaderOptions::new()).unwrap();
            let log: log_capnp::log::Reader = message_reader.get_root::<log_capnp::log::Reader>().unwrap();
            test::black_box(&log);
        });
    }

    /*
    #[bench]
    fn bench_serialize_packed_unbuffered(b: &mut Bencher) {
        let mut msg = Builder::new_default();
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
        let mut msg = Builder::new_default();
        new_log(&mut msg);
    
        let mut bytes = Vec::new();
        capnp::serialize_packed::write_packed_message_unbuffered(&mut bytes, &msg).unwrap();

        b.bytes = bytes.len() as u64;

        b.iter(|| {
            let mut rdr = bytes;
            let message_reader = capnp::serialize_packed::read_message(
                &mut rdr,
                ReaderOptions::new()
            ).unwrap();
            let _: log_capnp::log::Reader = message_reader.get_root::<log_capnp::log::Reader>();
        });
    }
    */
}

#[cfg(test)]
mod rustc_serialize_json {
    use test::{self, Bencher};
    use rustc_serialize::json;
    use rustc_serialize::Encodable;

    use super::Log;

    fn write_to_string(bytes: &mut String, log: &Log) {
        let mut encoder = json::Encoder::new(bytes);
        log.encode(&mut encoder).unwrap()
    }

    #[bench]
    fn bench_populate(b: &mut Bencher) {
        b.iter(|| {
            Log::new()
        });
    }

    #[bench]
    fn bench_encoder(b: &mut Bencher) {
        let log = Log::new();
        let mut bytes = String::new();
        write_to_string(&mut bytes, &log);
        b.bytes = bytes.len() as u64;

        b.iter(|| {
            bytes.clear();
            write_to_string(&mut bytes, &log);
            test::black_box(&bytes);
        });
    }

    #[bench]
    fn bench_decoder(b: &mut Bencher) {
        let log = Log::new();
        let json = json::encode(&log).unwrap();
        b.bytes = json.len() as u64;

        b.iter(|| {
            let log: Log = json::decode(&json).unwrap();
            log
        });
    }
}

#[cfg(test)]
mod serde_json {
    use test::{self, Bencher};

    use serde_json;

    use super::Log;

    #[bench]
    fn bench_populate(b: &mut Bencher) {
        b.iter(|| {
            Log::new()
        });
    }

    #[bench]
    fn bench_serializer(b: &mut Bencher) {
        let log = Log::new();
        let mut bytes = Vec::new();
        serde_json::to_writer(&mut bytes, &log).unwrap();
        b.bytes = bytes.len() as u64;

        b.iter(|| {
            bytes.clear();
            serde_json::to_writer(&mut bytes, &log).unwrap();
            test::black_box(&bytes);
        });
    }

    #[bench]
    fn bench_deserializer(b: &mut Bencher) {
        let log = Log::new();
        let json = serde_json::to_string(&log).unwrap();
        b.bytes = json.len() as u64;

        b.iter(|| {
            let log: Log = serde_json::from_str(&json).unwrap();
            log
        });
    }
}

#[cfg(test)]
mod msgpack {
    use test::{self, Bencher};
    use rustc_serialize::{Encodable, Decodable};

    use msgpack;

    use super::Log;

    fn write_to_bytes(bytes: &mut Vec<u8>, log: &Log) {
        let mut encoder = msgpack::Encoder::new(bytes);
        log.encode(&mut encoder).unwrap()
    }

    #[bench]
    fn bench_populate(b: &mut Bencher) {
        b.iter(|| {
            Log::new()
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
            test::black_box(&bytes);
        });
    }

    #[bench]
    fn bench_decoder(b: &mut Bencher) {
        let mut bytes = Vec::new();
        write_to_bytes(&mut bytes, &Log::new());
        b.bytes = bytes.len() as u64;

        b.iter(|| {
            let mut decoder = msgpack::Decoder::new(&*bytes);
            let log: Log = Decodable::decode(&mut decoder).unwrap();
            log
        });
    }
}

#[cfg(test)]
mod protobuf {
    use test::{self, Bencher};

    use protobuf::Message;
    use protobuf::Clear;
    use log_proto;

    fn new_log() -> log_proto::Log {
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

    #[bench]
    fn bench_populate(b: &mut Bencher) {
        b.iter(|| {
            new_log()
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
            test::black_box(&bytes);
        });
    }

    #[bench]
    fn bench_decoder(b: &mut Bencher) {
        let log = new_log();
        let bytes = log.write_to_bytes().unwrap();
        b.bytes = bytes.len() as u64;

        let mut log = log_proto::Log::new();
        b.iter(|| {
            log.clear();
            log.merge_from_bytes(&bytes).unwrap();
            assert!(log.is_initialized());
            test::black_box(&log);
        });
    }
}

#[cfg(test)]
mod bincode {
    use test::{self, Bencher};

    use bincode;

    use super::Log;

    #[bench]
    fn bench_populate(b: &mut Bencher) {
        b.iter(|| {
            Log::new()
        });
    }

    #[bench]
    fn bench_encoder(b: &mut Bencher) {
        let log = Log::new();
        let mut bytes = Vec::new();
        bincode::encode_into(&log, &mut bytes, bincode::SizeLimit::Infinite).unwrap();
        b.bytes = bytes.len() as u64;

        b.iter(|| {
            bytes.clear();
            bincode::encode_into(&log, &mut bytes, bincode::SizeLimit::Infinite).unwrap();
            test::black_box(&bytes);
        });
    }

    #[bench]
    fn bench_decoder(b: &mut Bencher) {
        let log = Log::new();
        let mut bytes = Vec::new();
        bincode::encode_into(&log, &mut bytes, bincode::SizeLimit::Infinite).unwrap();
        b.bytes = bytes.len() as u64;

        b.iter(|| {
            let log: Log = bincode::decode(&bytes).unwrap();
            log
        });
    }
}

#[cfg(test)]
mod bincode_serde {
    use test::{self, Bencher};

    use bincode;

    use super::Log;

    #[bench]
    fn bench_populate(b: &mut Bencher) {
        b.iter(|| {
            Log::new()
        });
    }

    #[bench]
    fn bench_serialize(b: &mut Bencher) {
        let log = Log::new();
        let mut bytes = bincode::to_vec(&log, bincode::SizeLimit::Infinite).unwrap();
        b.bytes = bytes.len() as u64;

        b.iter(|| {
            bytes.clear();
            bincode::to_writer(&mut bytes, &log, bincode::SizeLimit::Infinite).unwrap();
            test::black_box(&bytes);
        });
    }

    #[bench]
    fn bench_deserialize(b: &mut Bencher) {
        let log = Log::new();
        let bytes = bincode::to_vec(&log, bincode::SizeLimit::Infinite).unwrap();
        b.bytes = bytes.len() as u64;

        b.iter(|| {
            let log: Log = bincode::from_slice(&bytes, bincode::SizeLimit::Infinite).unwrap();
            log
        });
    }
}
