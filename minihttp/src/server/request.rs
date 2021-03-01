use super::*;
use crate::util::MemoryStream;
use alloc::string::*;
use bare_io::{Read, Write};
use core::result::Result;
use httparse::{Request, EMPTY_HEADER};

pub enum IncomingRequest {
    New {
        stream: MemoryStream,
    },
    Body {
        stream: MemoryStream,
        method: HttpMethod,
        path: String,
        headers: HttpHeaders,
        content_offset: usize,
        total_size: usize,
    },
}

impl Write for IncomingRequest {
    fn write(&mut self, buf: &[u8]) -> bare_io::Result<usize> {
        self.stream().write(buf)
    }

    fn flush(&mut self) -> bare_io::Result<()> {
        self.stream().flush()
    }
}

impl Read for IncomingRequest {
    fn read(&mut self, buf: &mut [u8]) -> bare_io::Result<usize> {
        self.stream().read(buf)
    }
}

impl IncomingRequest {
    fn stream(&mut self) -> &mut MemoryStream {
        match self {
            Self::New { stream } => stream,
            Self::Body { stream, .. } => stream,
        }
    }

    pub fn new() -> Self {
        Self::New {
            stream: MemoryStream::new(200),
        }
    }

    fn check_if_done<T>(&mut self) -> Result<bool, HttpError<T>> {
        match self {
            Self::New { stream } => {
                let mut headers = [EMPTY_HEADER; 64];
                let mut req = Request::new(&mut headers);
                let parsed = req.parse(stream.buf_ref());
                match parsed {
                    Ok(httparse::Status::Complete(s)) => {
                        let method = HttpMethod::from(req.method.unwrap_or_default());
                        let path = req.path.unwrap_or_default().to_string();
                        let headers = HttpHeaders::parse(&headers);
                        let total_size = s + headers.content_length().unwrap_or_default();
                        let stream = core::mem::replace(stream, MemoryStream::new(0));
                        *self = Self::Body {
                            content_offset: s,
                            method: method,
                            path: path,
                            stream,
                            headers,
                            total_size,
                        };
                        Ok(self.stream().buf_ref().len() >= total_size)
                    }
                    Ok(httparse::Status::Partial) => Ok(false),
                    Err(e) => Err(HttpError::ParseFailure(e)),
                }
            }
            Self::Body {
                stream, total_size, ..
            } => {
                if stream.buf_ref().len() >= *total_size {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }

    pub async fn read_from_stream<T: TcpClientStack>(
        &mut self,
        stack: &mut T,
        socket: &mut T::TcpSocket,
        buffer: &mut [u8],
    ) -> Result<bool, HttpError<T::Error>> {
        let sz = yield_nb!(stack.receive(socket, buffer))?;
        self.write(&buffer[..sz]).unwrap();
        self.check_if_done()
    }

    pub fn finish<T: TcpClientStack>(self) -> HttpResult<HttpRequest, T> {
        match self {
            Self::Body {
                total_size,
                content_offset,
                headers,
                method,
                path,
                stream,
            } => {
                let body = if total_size == content_offset {
                    None
                } else {
                    Some(stream.consume()[content_offset..].into())
                };
                let host = headers.host();
                Ok(HttpRequest {
                    method,
                    body,
                    headers,
                    host,
                    path,
                })
            }
            _ => Err(HttpError::EncodingFailure),
        }
    }
}
