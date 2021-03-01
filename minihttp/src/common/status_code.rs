use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpStatus {
    Accepted,
    BadGateway,
    BadRequest,
    Conflict,
    Continue,
    Created,
    ExpectationFailed,
    Forbidden,
    Found,
    GatewayTimeout,
    Gone,
    HttpVersionNotSupported,
    InternalServerError,
    LengthRequired,
    MethodNotAllowed,
    MovedPermanently,
    MultipleChoices,
    NoContent,
    NonAuthoritativeInformation,
    NotAcceptable,
    NotFound,
    NotImplemented,
    NotModified,
    OK,
    PartialContent,
    PaymentRequired,
    PreconditionFailed,
    ProxyAuthenticationRequired,
    RequestEntityTooLarge,
    RequestTimeout,
    RequestUriTooLong,
    RequestedRangeNotSatisfiable,
    ResetContent,
    SeeOther,
    ServiceUnavailable,
    SwitchingProtocols,
    TemporaryRedirect,
    Unauthorized,
    UnsupportedMediaType,
    UseProxy,
    Other(u16),
}

impl From<u16> for HttpStatus {
    fn from(code: u16) -> Self {
        match code {
            100 => Self::Continue,
            101 => Self::SwitchingProtocols,
            200 => Self::OK,
            201 => Self::Created,
            202 => Self::Accepted,
            203 => Self::NonAuthoritativeInformation,
            204 => Self::NoContent,
            205 => Self::ResetContent,
            206 => Self::PartialContent,
            300 => Self::MultipleChoices,
            301 => Self::MovedPermanently,
            302 => Self::Found,
            303 => Self::SeeOther,
            304 => Self::NotModified,
            305 => Self::UseProxy,
            307 => Self::TemporaryRedirect,
            400 => Self::BadRequest,
            401 => Self::Unauthorized,
            402 => Self::PaymentRequired,
            403 => Self::Forbidden,
            404 => Self::NotFound,
            405 => Self::MethodNotAllowed,
            406 => Self::NotAcceptable,
            407 => Self::ProxyAuthenticationRequired,
            408 => Self::RequestTimeout,
            409 => Self::Conflict,
            410 => Self::Gone,
            411 => Self::LengthRequired,
            412 => Self::PreconditionFailed,
            413 => Self::RequestEntityTooLarge,
            414 => Self::RequestUriTooLong,
            415 => Self::UnsupportedMediaType,
            416 => Self::RequestedRangeNotSatisfiable,
            417 => Self::ExpectationFailed,
            500 => Self::InternalServerError,
            501 => Self::NotImplemented,
            502 => Self::BadGateway,
            503 => Self::ServiceUnavailable,
            504 => Self::GatewayTimeout,
            505 => Self::HttpVersionNotSupported,
            code => Self::Other(code),
        }
    }
}

impl fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Accepted => write!(f, "202 Accepted"),
            Self::BadGateway => write!(f, "502 Bad Gateway"),
            Self::BadRequest => write!(f, "400 Bad Request"),
            Self::Conflict => write!(f, "409 Conflict"),
            Self::Continue => write!(f, "100 Continue"),
            Self::Created => write!(f, "201 Created"),
            Self::ExpectationFailed => write!(f, "417 Expectation Failed"),
            Self::Forbidden => write!(f, "403 Forbidden"),
            Self::Found => write!(f, "302 Found"),
            Self::GatewayTimeout => write!(f, "504 Gateway Timeout"),
            Self::Gone => write!(f, "410 Gone"),
            Self::HttpVersionNotSupported => write!(f, "505 HTTP Version Not Supported"),
            Self::InternalServerError => write!(f, "500 Internal Server Error"),
            Self::LengthRequired => write!(f, "411 Length Required"),
            Self::MethodNotAllowed => write!(f, "405 Method Not Allowed"),
            Self::MovedPermanently => write!(f, "301 Moved Permanently"),
            Self::MultipleChoices => write!(f, "300 Multiple Choices"),
            Self::NoContent => write!(f, "204 No Content"),
            Self::NonAuthoritativeInformation => {
                write!(f, "203 No Authoritative Information")
            }
            Self::NotAcceptable => write!(f, "406 Not Acceptable"),
            Self::NotFound => write!(f, "404 Not Found"),
            Self::NotImplemented => write!(f, "501 Not Implemented"),
            Self::NotModified => write!(f, "304 NotModified"),
            Self::OK => write!(f, "200 OK"),
            Self::PartialContent => write!(f, "206 Partial Content"),
            Self::PaymentRequired => write!(f, "402 Payment Required"),
            Self::PreconditionFailed => write!(f, "412 Precondition Failed"),
            Self::ProxyAuthenticationRequired => {
                write!(f, "407 Proxy Authentication Required")
            }
            Self::RequestEntityTooLarge => write!(f, "413 Request Entity Too Large"),
            Self::RequestTimeout => write!(f, "408 Request Timeout"),
            Self::RequestUriTooLong => write!(f, "414 Request URI Too Long"),
            Self::RequestedRangeNotSatisfiable => {
                write!(f, "416 Requested Range Not Satisfiable")
            }
            Self::ResetContent => write!(f, "205 Reset Content"),
            Self::SeeOther => write!(f, "303 See Other"),
            Self::ServiceUnavailable => write!(f, "503 Service Unavailable"),
            Self::SwitchingProtocols => write!(f, "101 Switching Protocols"),
            Self::TemporaryRedirect => write!(f, "307 Temporary Redirect"),
            Self::Unauthorized => write!(f, "401 Unauthorized"),
            Self::UnsupportedMediaType => write!(f, "415 Unsupported Media Type"),
            Self::UseProxy => write!(f, "305 Use Proxy"),
            Self::Other(code) => write!(f, "{} I am a teapot", code),
        }
    }
}
