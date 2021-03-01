use core::fmt;

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    Unknown,
}

impl HttpMethod {
    pub const fn text(&self) -> &str {
        match self {
            Self::GET => "GET",
            Self::POST => "POST",
            Self::DELETE => "DELETE",
            Self::PUT => "PUT",
            Self::HEAD => "HEAD",
            Self::OPTIONS => "OPTIONS",
            Self::PATCH => "PATCH",
            Self::Unknown => "",
        }
    }

    pub const fn request_body_allowed(&self) -> bool {
        match self {
            Self::POST | Self::PUT | Self::PATCH | Self::Unknown => true,
            Self::GET | Self::DELETE | Self::HEAD | Self::OPTIONS => false,
        }
    }

    pub const fn response_body_allowed(&self) -> bool {
        match self {
            Self::HEAD => false,
            _ => true,
        }
    }
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text())
    }
}

impl From<&str> for HttpMethod {
    fn from(method: &str) -> Self {
        match method.to_uppercase().as_str() {
            "GET" => Self::GET,
            "POST" => Self::POST,
            "DELETE" => Self::DELETE,
            "PUT" => Self::PUT,
            "HEAD" => Self::HEAD,
            "OPTIONS" => Self::OPTIONS,
            "PATCH" => Self::PATCH,
            _ => Self::Unknown,
        }
    }
}
