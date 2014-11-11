#include <cstdio>
#include <string>
#include <vector>
#include <iostream>

#include "rapidjson/error/en.h"
#include "rapidjson/document.h"
#include "rapidjson/reader.h"
#include "rapidjson/writer.h"
#include "rapidjson/stringbuffer.h"

#include "bench.h"

using namespace std;
using namespace rapidjson;

enum HttpProtocol {
    HTTP_PROTOCOL_UNKNOWN,
    HTTP10,
    HTTP11,
};

enum HttpMethod {
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
};

struct Http {
    HttpProtocol protocol;
    unsigned status;
    unsigned host_status;
    unsigned up_status;
    HttpMethod method;
    std::string content_type;
    std::string user_agent;
    std::string referer;
    std::string request_uri;

    template <typename Writer>
    void Serialize(Writer& writer) const {
      writer.StartObject();

      writer.String("protocol");
      writer.Uint(protocol);

      writer.String("status");
      writer.Uint(status);

      writer.String("host_status");
      writer.Uint(host_status);

      writer.String("up_status");
      writer.Uint(up_status);

      writer.String("method");
      writer.Uint(method);

      writer.String("content_type");
      writer.String(content_type.c_str());

      writer.String("user_agent");
      writer.String(user_agent.c_str());

      writer.String("referer");
      writer.String(referer.c_str());

      writer.String("request_uri");
      writer.String(request_uri.c_str());

      writer.EndObject();
    }

    template<typename Doc>
    void Deserialize(Doc& document) {
      protocol = static_cast<HttpProtocol>(document["protocol"].GetUint());
      status = document["status"].GetUint();
      host_status = document["host_status"].GetUint();
      up_status = document["up_status"].GetUint();
      method = static_cast<HttpMethod>(document["method"].GetUint());
      content_type = document["content_type"].GetString();
      user_agent = document["user_agent"].GetString();
      referer = document["referer"].GetString();
      request_uri = document["request_uri"].GetString();
    }

    enum State {
      ExpectObjectStart,
      ExpectNameOrObjectEnd,
      ExpectProtocol,
      ExpectStatus,
      ExpectHostStatus,
      ExpectUpStatus,
      ExpectMethod,
      ExpectContentType,
      ExpectUserAgent,
      ExpectReferer,
      ExpectRequestUri,
    };

    struct Handler: BaseReaderHandler<> {
      Http& http;
      State state;

      Handler(Http& http): http(http), state(ExpectObjectStart) {}

      bool StartObject() {
        switch (state) {
          case ExpectObjectStart:
            state = ExpectNameOrObjectEnd;
            return true;
          default:
            return false;
        }
      }

      bool Int(int value) {
        return Uint64(value);
      }

      bool Int64(int64_t value) {
        return Uint64(value);
      }

      bool Uint(int value) {
        return Uint64(value);
      }

      bool Uint64(uint64_t value) {
        switch (state) {
          case ExpectProtocol:
            http.protocol = static_cast<HttpProtocol>(value);
            state = ExpectNameOrObjectEnd;
            return true;

          case ExpectStatus:
            http.status = value;
            state = ExpectNameOrObjectEnd;
            return true;

          case ExpectUpStatus:
            http.up_status = value;
            state = ExpectNameOrObjectEnd;
            return true;

          case ExpectHostStatus:
            http.host_status = value;
            state = ExpectNameOrObjectEnd;
            return true;

          case ExpectMethod:
            http.method = static_cast<HttpMethod>(value);
            state = ExpectNameOrObjectEnd;
            return true;

          default:
            return false;
        }
      }

      bool String(const char* str, SizeType length, bool) {
        switch (state) {
          case ExpectContentType:
            http.content_type = std::string(str, length);
            state = ExpectNameOrObjectEnd;
            return true;

          case ExpectUserAgent:
            http.user_agent = std::string(str, length);
            state = ExpectNameOrObjectEnd;
            return true;

          case ExpectReferer:
            http.referer = std::string(str, length);
            state = ExpectNameOrObjectEnd;
            return true;

          case ExpectRequestUri:
            http.request_uri = std::string(str, length);
            state = ExpectNameOrObjectEnd;
            return true;

          default:
            return false;
        }
      }

      bool Key(const char* str, SizeType length, bool) {
        switch (state) {
          case ExpectNameOrObjectEnd:
            if (memcmp(str, "protocol", sizeof("protocol")) == 0) {
              state = ExpectProtocol;
            } else if (memcmp(str, "status", sizeof("status")) == 0) {
              state = ExpectStatus;
            } else if (memcmp(str, "host_status", sizeof("host_status")) == 0) {
              state = ExpectHostStatus;
            } else if (memcmp(str, "up_status", sizeof("up_status")) == 0) {
              state = ExpectUpStatus;
            } else if (memcmp(str, "method", sizeof("method")) == 0) {
              state = ExpectMethod;
            } else if (memcmp(str, "content_type", sizeof("content_type")) == 0) {
              state = ExpectContentType;
            } else if (memcmp(str, "user_agent", sizeof("user_agent")) == 0) {
              state = ExpectUserAgent;
            } else if (memcmp(str, "referer", sizeof("referer")) == 0) {
              state = ExpectReferer;
            } else if (memcmp(str, "request_uri", sizeof("request_uri")) == 0) {
              state = ExpectRequestUri;
            } else {
              return false;
            }

            return true;
          default:
            return false;
        }
      }

      bool EndObject(SizeType) {
        return state == ExpectNameOrObjectEnd;
      }

      bool Default() {
        return false;
      }
    };
};

enum CacheStatus {
    CACHESTATUS_UNKNOWN,
    Miss,
    Expired,
    Hit,
};

enum OriginProtocol {
    ORIGIN_PROTOCOL_UNKNOWN,
    HTTP,
    HTTPS,
};

struct Origin {
    std::string ip;
    unsigned port;
    std::string hostname;
    OriginProtocol protocol;

    template <typename Writer>
    void Serialize(Writer& writer) const {
      writer.StartObject();

      writer.String("ip");
      writer.String(ip.c_str());

      writer.String("port");
      writer.Uint(port);

      writer.String("hostname");
      writer.String(hostname.c_str());

      writer.String("protocol");
      writer.Uint(protocol);

      writer.EndObject();
    }

    template<typename Doc>
    void Deserialize(Doc& document) {
      ip = document["ip"].GetString();
      port = document["port"].GetUint();
      hostname = document["hostname"].GetString();
      protocol = static_cast<OriginProtocol>(document["protocol"].GetUint());
    }

    enum State {
      ExpectObjectStart,
      ExpectNameOrObjectEnd,
      ExpectIp,
      ExpectPort,
      ExpectHostname,
      ExpectProtocol,
    };

    struct Handler: BaseReaderHandler<> {
      Origin& origin;
      State state;

      Handler(Origin& origin): origin(origin), state(ExpectObjectStart) {}

      bool StartObject() {
        switch (state) {
          case ExpectObjectStart:
            state = ExpectNameOrObjectEnd;
            return true;
          default:
            return false;
        }
      }

      bool Int(int value) {
        return Uint64(value);
      }

      bool Int64(int64_t value) {
        return Uint64(value);
      }

      bool Uint(int value) {
        return Uint64(value);
      }

      bool Uint64(uint64_t value) {
        switch (state) {
          case ExpectPort:
            origin.port = value;
            state = ExpectNameOrObjectEnd;
            return true;

          case ExpectProtocol:
            origin.protocol = static_cast<OriginProtocol>(value);
            state = ExpectNameOrObjectEnd;
            return true;

          default:
            return false;
        }
      }

      bool String(const char* str, SizeType length, bool) {
        switch (state) {
          case ExpectIp:
            origin.ip = std::string(str, length);
            state = ExpectNameOrObjectEnd;
            return true;

          case ExpectHostname:
            origin.hostname = std::string(str, length);
            state = ExpectNameOrObjectEnd;
            return true;

          default:
            return false;
        }
      }

      bool Key(const char* str, SizeType length, bool) {
        switch (state) {
          case ExpectNameOrObjectEnd:
            if (memcmp(str, "ip", sizeof("ip")) == 0) {
              state = ExpectIp;
            } else if (memcmp(str, "port", sizeof("port")) == 0) {
              state = ExpectPort;
            } else if (memcmp(str, "hostname", sizeof("hostname")) == 0) {
              state = ExpectHostname;
            } else if (memcmp(str, "protocol", sizeof("protocol")) == 0) {
              state = ExpectProtocol;
            } else {
              return false;
            }

            return true;
          default:
            return false;
        }
      }

      bool EndObject(SizeType) {
        return state == ExpectNameOrObjectEnd;
      }

      bool Default() {
        return false;
      }
    };
};

enum ZonePlan {
    ZONEPLAN_UNKNOWN,
    FREE,
    PRO,
    BIZ,
    ENT,
};

enum Country {
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
};

struct Log {
    long timestamp;
    unsigned zone_id;
    ZonePlan zone_plan;
    Http http;
    Origin origin;
    Country country;
    CacheStatus cache_status;
    std::string server_ip;
    std::string server_name;
    std::string remote_ip;
    unsigned long bytes_dlv;
    std::string ray_id;

    template <typename Writer>
    void Serialize(Writer& writer) const {
      writer.StartObject();

      writer.String("timestamp");
      writer.Int64(timestamp);

      writer.String("zone_id");
      writer.Uint(zone_id);

      writer.String("zone_plan");
      writer.Uint(zone_plan);

      writer.String("http");
      http.Serialize(writer);

      writer.String("origin");
      origin.Serialize(writer);

      writer.String("country");
      writer.Uint(country);

      writer.String("cache_status");
      writer.Uint(cache_status);

      writer.String("server_ip");
      writer.String(server_ip.c_str());

      writer.String("server_name");
      writer.String(server_name.c_str());

      writer.String("remote_ip");
      writer.String(remote_ip.c_str());

      writer.String("bytes_dlv");
      writer.Uint(bytes_dlv);

      writer.String("ray_id");
      writer.String(ray_id.c_str());

      writer.EndObject();
    }

    void Deserialize(Document& document) {
      timestamp = document["timestamp"].GetInt64();
      zone_id = document["zone_id"].GetUint();
      zone_plan = static_cast<ZonePlan>(document["zone_plan"].GetUint());
      http.Deserialize(document["http"]);
      origin.Deserialize(document["origin"]);
      country = static_cast<Country>(document["country"].GetUint());
      cache_status = static_cast<CacheStatus>(document["cache_status"].GetUint());
      server_ip = document["server_ip"].GetString();
      server_name = document["server_name"].GetString();
      remote_ip = document["remote_ip"].GetString();
      bytes_dlv = document["bytes_dlv"].GetUint();
      ray_id = document["ray_id"].GetString();
    }

    enum State {
      ExpectObjectStart,
      ExpectNameOrObjectEnd,
      ExpectTimestamp,
      ExpectZoneId,
      ExpectZonePlan,
      ExpectHttp,
      HandleHttp,
      ExpectOrigin,
      HandleOrigin,
      ExpectCountry,
      ExpectCacheStatus,
      ExpectServerIp,
      ExpectServerName,
      ExpectRemoteIp,
      ExpectBytesDlv,
      ExpectRayId,
    };

    struct Handler: BaseReaderHandler<> {
      Log& log;
      Http::Handler http_handler;
      Origin::Handler origin_handler;
      State state;

      Handler(Log& log): log(log), http_handler(log.http), origin_handler(log.origin), state(ExpectObjectStart) {}

      bool StartObject() {
        switch (state) {
          case ExpectObjectStart:
            state = ExpectNameOrObjectEnd;
            return true;

          case ExpectHttp:
            http_handler.StartObject();
            state = HandleHttp;
            return true;

          case ExpectOrigin:
            origin_handler.StartObject();
            state = HandleOrigin;
            return true;

          default:
            return false;
        }
      }

      bool Int(int value) {
        return Uint64(value);
      }

      bool Int64(int64_t value) {
        return Uint64(value);
      }

      bool Uint(uint value) {
        return Uint64(value);
      }


      bool Uint64(uint64_t value) {
        switch (state) {
          case HandleHttp:
            return http_handler.Uint64(value);

          case HandleOrigin:
            return origin_handler.Uint64(value);

          case ExpectTimestamp:
            log.timestamp = value;
            state = ExpectNameOrObjectEnd;
            return true;

          case ExpectZoneId:
            log.zone_id = value;
            state = ExpectNameOrObjectEnd;
            return true;

          case ExpectZonePlan:
            log.zone_plan = static_cast<ZonePlan>(value);
            state = ExpectNameOrObjectEnd;
            return true;

          case ExpectCountry:
            log.country = static_cast<Country>(value);
            state = ExpectNameOrObjectEnd;
            return true;

          case ExpectCacheStatus:
            log.cache_status = static_cast<CacheStatus>(value);
            state = ExpectNameOrObjectEnd;
            return true;

          case ExpectBytesDlv:
            log.bytes_dlv = value;
            state = ExpectNameOrObjectEnd;
            return true;

          default:
            abort();
            return false;
        }
      }

      bool Double(double value) {
        return Uint(value);
      }

      bool String(const char* str, SizeType length, bool copy) {
        switch (state) {
          case HandleHttp:
            return http_handler.String(str, length, copy);

          case HandleOrigin:
            return origin_handler.String(str, length, copy);

          case ExpectServerIp:
            log.server_ip = std::string(str, length);
            state = ExpectNameOrObjectEnd;
            return true;

          case ExpectServerName:
            log.server_name = std::string(str, length);
            state = ExpectNameOrObjectEnd;
            return true;

          case ExpectRemoteIp:
            log.remote_ip = std::string(str, length);
            state = ExpectNameOrObjectEnd;
            return true;

          case ExpectRayId:
            log.ray_id = std::string(str, length);
            state = ExpectNameOrObjectEnd;
            return true;

          default:
            return false;
        }
      }

      bool Key(const char* str, SizeType length, bool copy) {
        switch (state) {
          case ExpectNameOrObjectEnd:
            if (memcmp(str, "timestamp", sizeof("timestamp")) == 0) {
              state = ExpectTimestamp;
            } else if (memcmp(str, "zone_id", sizeof("zone_id")) == 0) {
              state = ExpectZoneId;
            } else if (memcmp(str, "zone_plan", sizeof("zone_plan")) == 0) {
              state = ExpectZonePlan;
            } else if (memcmp(str, "http", sizeof("http")) == 0) {
              state = ExpectHttp;
            } else if (memcmp(str, "origin", sizeof("origin")) == 0) {
              state = ExpectOrigin;
            } else if (memcmp(str, "country", sizeof("country")) == 0) {
              state = ExpectCountry;
            } else if (memcmp(str, "cache_status", sizeof("cache_status")) == 0) {
              state = ExpectCacheStatus;
            } else if (memcmp(str, "server_ip", sizeof("server_ip")) == 0) {
              state = ExpectServerIp;
            } else if (memcmp(str, "server_name", sizeof("server_name")) == 0) {
              state = ExpectServerName;
            } else if (memcmp(str, "remote_ip", sizeof("remote_ip")) == 0) {
              state = ExpectRemoteIp;
            } else if (memcmp(str, "bytes_dlv", sizeof("bytes_dlv")) == 0) {
              state = ExpectBytesDlv;
            } else if (memcmp(str, "ray_id", sizeof("ray_id")) == 0) {
              state = ExpectRayId;
            } else {
              return false;
            }

            return true;

          case HandleHttp:
            return http_handler.Key(str, length, copy);

          case HandleOrigin:
            return origin_handler.Key(str, length, copy);

          default:
            return false;
        }
      }

      bool EndObject(SizeType length) {
        switch (state) {
          case ExpectNameOrObjectEnd:
            return true;

          case HandleHttp:
            state = ExpectNameOrObjectEnd;
            return http_handler.EndObject(length);

          case HandleOrigin:
            state = ExpectNameOrObjectEnd;
            return origin_handler.EndObject(length);

          default:
            return false;
        }
      }

      bool Default() {
        return false;
      }
    };
};

std::string log_serialize(Log& log) {
  StringBuffer s(0, 1024);
  Writer<StringBuffer> writer(s);
  log.Serialize(writer);

  return s.GetString();
}

void bench_log_serialize(Log& log) {
  log_serialize(log);
}

void bench_log_deserialize_dom(string& json) {
  Document document;
  document.Parse(json.c_str());
  Log log;
  log.Deserialize(document);
}

void bench_log_deserialize_sax(string& json) {
  Reader reader;
  Log log;
  Log::Handler handler(log);
  StringStream ss(json.c_str());
  if (!reader.Parse(ss, handler)) {
    ParseErrorCode e = reader.GetParseErrorCode();
    size_t o = reader.GetErrorOffset();
    cout << "Error: " << GetParseError_En(e) << endl;
    exit(1);
  }
}

string make_str() {
  string s;
  for (int i = 0; i < 1024; i++) {
    s.push_back('a');
  }
  return s;
}

string make_json_str() {
  string s;
  s.push_back('"');
  for (int i = 0; i < 1024; i++) {
    s.push_back('a');
  }
  s.push_back('"');
  return s;
}

string str_serialize(string& str) {
  StringBuffer s(0, 1024);
  Writer<StringBuffer> writer(s);
  writer.String(str.c_str(), str.size());

  return s.GetString();
}

void bench_str_serialize(string& str) {
  str_serialize(str);
}

void bench_str_deserialize_dom(string& json) {
  Document document;
  document.Parse(json.c_str());
  document.GetString();
}

struct StringHandler: BaseReaderHandler<> {
  string str;

  StringHandler(): str("") {}

  bool String(const char* str, SizeType length, bool copy) {
    this->str = string(str, length);

    return true;
  }
};

void bench_str_deserialize_sax(string& json) {
  Reader reader;
  StringHandler handler;
  StringStream ss(json.c_str());
  if (!reader.Parse(ss, handler)) {
    ParseErrorCode e = reader.GetParseErrorCode();
    size_t o = reader.GetErrorOffset();
    cout << "Error: " << GetParseError_En(e) << endl;
    exit(1);
  }
}

int main(int, char*[]) {
  Http http = {
    HTTP11,
    200,
    503,
    520,
    GET,
    "text/html",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/33.0.1750.146 Safari/537.36",
    "https://www.cloudflare.com/",
    "/cdn-cgi/trace"
  };

  Origin origin = {
    "1.2.3.4",
    8000,
    "www.example.com",
    HTTPS
  };

  Log log = {
    2837513946597,
    123,
    FREE,
    http,
    origin,
    US,
    Hit,
    "192.168.1.1",
    "metal.cloudflare.com",
    "10.1.2.3",
    456,
    "10c73629cce30078-LAX",
  };

  string log_json = log_serialize(log);
  bench("log serialize      ", log, log_json.size(), bench_log_serialize);
  bench("log dom deserialize", log_json, log_json.size(), bench_log_deserialize_dom);
  bench("log sax deserialize", log_json, log_json.size(), bench_log_deserialize_sax);

  string str = make_str();
  string str_json = str_serialize(str);
  bench("str serialize      ", str, str_json.size(), bench_str_serialize);
  bench("str dom deserialize", str_json, str_json.size(), bench_str_deserialize_dom);
  bench("str sax deserialize", str_json, str_json.size(), bench_str_deserialize_sax);

	return 0;
}
