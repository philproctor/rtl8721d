use alloc::string::*;

#[derive(Debug, Clone)]
pub enum ContentType {
    ApplicationJson,
    ApplicationGzip,
    ImagePng,
    ImageJpg,
    ImageGif,
    TextPlain,
    TextHtml,
    TextCss,
    TextJavascript,
    Undefined,
    Other(String),
}

impl ContentType {
    pub fn text(&self) -> &str {
        match self {
            Self::ApplicationJson => "application/json",
            Self::ApplicationGzip => "application/x-gzip",
            Self::ImagePng => "image/png",
            Self::ImageJpg => "image/jpeg",
            Self::ImageGif => "image/gif",
            Self::TextPlain => "text/plain",
            Self::TextHtml => "text/html",
            Self::TextCss => "text/css",
            Self::TextJavascript => "text/javascript",
            Self::Undefined => "application/octet-stream",
            Self::Other(s) => s.as_str(),
        }
    }
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        match content_type.to_lowercase().as_str() {
            "text/html" => Self::TextHtml,
            "text/plain" => Self::TextPlain,
            "text/javascript" | "application/javascript" => Self::TextJavascript,
            "text/css" => Self::TextCss,
            "application/json" | "text/json" => Self::ApplicationJson,
            "application/x-gzip" | "application/gzip" => Self::ApplicationGzip,
            "image/png" => Self::ImagePng,
            "image/jpeg" | "image/jpg" => Self::ImageJpg,
            "image/gif" => Self::ImageGif,
            other => Self::Other(other.to_string()),
        }
    }
}
