use serde::Deserialize;
use std::collections::HashMap;
use std::{fmt, string::FromUtf8Error};

#[derive(Default, Clone, Debug)]
pub struct Response {
    pub headers: HashMap<String, Vec<u8>>,
    pub content: Vec<u8>,
    pub status_code: StatusCode,
}

impl Response {
    pub fn text(&self) -> Result<String, FromUtf8Error> {
        let result = if self.content.is_empty() {
            String::new()
        } else {
            String::from_utf8(self.content.clone())?
        };
        Ok(result)
    }

    pub fn json<'a, T: Deserialize<'a>>(&'a self) -> serde_json::Result<T> {
        serde_json::from_slice::<T>(&self.content[..])
    }

    pub fn is_success(&self) -> bool {
        self.status_code.is_success()
    }
}

macro_rules! impl_status_code {
    (
        $(
            $(#[$attrs:meta])*
            $name:ident = $value:expr
        ),+ $(,)?
    ) => {
        #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(u16)]
        pub enum StatusCode {
            $(
                $(#[$attrs])*
                // #[doc = stringify!($value)]
                $name = $value,
            )+
        }

        impl TryFrom<u16> for StatusCode {
            type Error = UnsupportedStatusCode;

            fn try_from(value: u16) -> Result<Self, Self::Error> {
                match value {
                    $(
                        $value => Ok(StatusCode::$name),
                    )+
                    _ => Err(UnsupportedStatusCode(value)),
                }
            }
        }
    };
}

#[derive(Debug, Clone, Copy, thiserror::Error)]
#[error("Unsupported status code: {0}")]
pub struct UnsupportedStatusCode(pub u16);

impl From<UnsupportedStatusCode> for u16 {
    fn from(err: UnsupportedStatusCode) -> Self {
        err.0
    }
}

impl From<u16> for UnsupportedStatusCode {
    fn from(value: u16) -> Self {
        UnsupportedStatusCode(value)
    }
}

impl_status_code! {
    /// 99
    NoInternet = 99,
    /// 100
    Continue = 100,
    /// 101
    SwitchingProtocols = 101,
    /// 200
    Ok = 200,
    /// 201
    Created = 201,
    /// 202
    Accepted = 202,
    /// 203
    NonAuthoritativeInformation = 203,
    /// 204
    NoContent = 204,
    /// 205
    ResetContent = 205,
    /// 206
    PartialContent = 206,
    /// 300
    MultipleChoices = 300,
    /// 301
    MovedPermanently = 301,
    /// 302
    Found = 302,
    /// 303
    SeeOther = 303,
    /// 304
    NotModified = 304,
    /// 307
    TemporaryRedirect = 307,
    /// 308
    PermanentRedirect = 308,
    /// 400
    BadRequest = 400,
    /// 401
    Unauthorized = 401,
    /// 403
    Forbidden = 403,
    /// 404
    #[default]
    NotFound = 404,
    /// 405
    MethodNotAllowed = 405,
    /// 406
    NotAcceptable = 406,
    /// 407
    ProxyAuthenticationRequired = 407,
    /// 408
    RequestTimeout = 408,
    /// 409
    Conflict = 409,
    /// 410
    Gone = 410,
    /// 411
    LengthRequired = 411,
    /// 412
    PreconditionFailed = 412,
    /// 413
    PayloadTooLarge = 413,
    /// 414
    UriTooLong = 414,
    /// 415
    UnsupportedMediaType = 415,
    /// 416
    RangeNotSatisfiable = 416,
    /// 417
    ExpectationFailed = 417,
    /// 418
    ImATeapot = 418,
    /// 421
    MisdirectedRequest = 421,
    /// 422
    UnprocessableEntity = 422,
    /// 426
    UpgradeRequired = 426,
    /// 428
    PreconditionRequired = 428,
    /// 429
    TooManyRequests = 429,
    /// 431
    RequestHeaderFieldsTooLarge = 431,
    /// 451
    UnavailableForLegalReasons = 451,
    /// 500
    InternalServerError = 500,
    /// 501
    NotImplemented = 501,
    /// 502
    BadGateway = 502,
    /// 503
    ServiceUnavailable = 503,
    /// 504
    GatewayTimeout = 504,
    /// 505
    HttpVersionNotSupported = 505,
    /// 506
    VariantAlsoNegotiates = 506,
    /// 510
    NotExtended = 510,
    /// 511
    NetworkAuthenticationRequired = 511,
}

impl StatusCode {
    pub fn is_success(&self) -> bool {
        (StatusCode::Ok..StatusCode::MultipleChoices).contains(self)
    }

    #[inline(always)]
    pub const fn as_u16(self) -> u16 {
        self as u16
    }
}

impl From<StatusCode> for u16 {
    fn from(status_code: StatusCode) -> Self {
        status_code as u16
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use StatusCode::*;
        match self {
            NoInternet => write!(f, "No Internet"),
            Continue => write!(f, "Continue"),
            SwitchingProtocols => write!(f, "Switching Protocols"),
            Ok => write!(f, "OK"),
            Created => write!(f, "Created"),
            Accepted => write!(f, "Accepted"),
            NonAuthoritativeInformation => write!(f, "Non-Authoritative Information"),
            NoContent => write!(f, "No Content"),
            ResetContent => write!(f, "Reset Content"),
            PartialContent => write!(f, "Partial Content"),
            MultipleChoices => write!(f, "Multiple Choices"),
            MovedPermanently => write!(f, "Moved Permanently"),
            Found => write!(f, "Found"),
            SeeOther => write!(f, "See Other"),
            NotModified => write!(f, "Not Modified"),
            TemporaryRedirect => write!(f, "Temporary Redirect"),
            PermanentRedirect => write!(f, "Permanent Redirect"),
            BadRequest => write!(f, "Bad Request"),
            Unauthorized => write!(f, "Unauthorized"),
            Forbidden => write!(f, "Forbidden"),
            NotFound => write!(f, "Not Found"),
            MethodNotAllowed => write!(f, "Method Not Allowed"),
            NotAcceptable => write!(f, "Not Acceptable"),
            ProxyAuthenticationRequired => write!(f, "Proxy Authentication Required"),
            RequestTimeout => write!(f, "Request Timeout"),
            Conflict => write!(f, "Conflict"),
            Gone => write!(f, "Gone"),
            LengthRequired => write!(f, "Length Required"),
            PreconditionFailed => write!(f, "Precondition Failed"),
            PayloadTooLarge => write!(f, "Payload Too Large"),
            UriTooLong => write!(f, "URI Too Long"),
            UnsupportedMediaType => write!(f, "Unsupported Media Type"),
            RangeNotSatisfiable => write!(f, "Range Not Satisfiable"),
            ExpectationFailed => write!(f, "Expectation Failed"),
            ImATeapot => write!(f, "I'm a teapot"),
            MisdirectedRequest => write!(f, "Misdirected Request"),
            UpgradeRequired => write!(f, "Upgrade Required"),
            PreconditionRequired => write!(f, "Precondition Required"),
            TooManyRequests => write!(f, "Too Many Requests"),
            RequestHeaderFieldsTooLarge => write!(f, "Request Header Fields Too Large"),
            UnavailableForLegalReasons => write!(f, "Unavailable For Legal Reasons"),
            InternalServerError => write!(f, "Internal Server Error"),
            NotImplemented => write!(f, "Not Implemented"),
            BadGateway => write!(f, "Bad Gateway"),
            ServiceUnavailable => write!(f, "Service Unavailable"),
            GatewayTimeout => write!(f, "Gateway Timeout"),
            HttpVersionNotSupported => write!(f, "HTTP Version Not Supported"),
            VariantAlsoNegotiates => write!(f, "Variant Also Negotiates"),
            NotExtended => write!(f, "Not Extended"),
            NetworkAuthenticationRequired => write!(f, "Network Authentication Required"),
            UnprocessableEntity => write!(f, "Unprocessable Entity")
        }
    }
}
