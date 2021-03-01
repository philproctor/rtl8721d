use embedded_nal::TcpClientStack;

pub type HttpResult<T, U> = core::result::Result<T, HttpError<<U as TcpClientStack>::Error>>;

#[derive(Debug)]
pub enum HttpError<T> {
    Timeout(u32),
    Status(i16),
    Network(T),
    ParseFailure(httparse::Error),
    Io(bare_io::Error),
    UnknownHost,
    EncodingFailure,
}

impl<T> From<T> for HttpError<T> {
    fn from(e: T) -> Self {
        Self::Network(e)
    }
}
