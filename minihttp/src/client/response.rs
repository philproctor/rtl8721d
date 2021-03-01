use super::*;
use crate::util::MemoryStream;
use bare_io::{Read, Write};
use core::result::Result;
use httparse::{Response, EMPTY_HEADER};

pub enum IncomingResponse {
    New {
        stream: MemoryStream,
    },
    Body {
        stream: MemoryStream,
        headers: HttpHeaders,
        content_offset: usize,
        total_size: usize,
        status: u16,
    },
}

impl Write for IncomingResponse {
    fn write(&mut self, buf: &[u8]) -> bare_io::Result<usize> {
        self.stream().write(buf)
    }

    fn flush(&mut self) -> bare_io::Result<()> {
        self.stream().flush()
    }
}

impl Read for IncomingResponse {
    fn read(&mut self, buf: &mut [u8]) -> bare_io::Result<usize> {
        self.stream().read(buf)
    }
}

impl IncomingResponse {
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
                let mut rsp = Response::new(&mut headers);
                let parsed = rsp.parse(stream.buf_ref());
                match parsed {
                    Ok(httparse::Status::Complete(s)) => {
                        let status = rsp.code.unwrap_or_default();
                        let headers = HttpHeaders::parse(&headers);
                        let total_size = s + headers.content_length().unwrap_or_default();
                        let stream = core::mem::replace(stream, MemoryStream::new(0));
                        *self = Self::Body {
                            content_offset: s,
                            stream,
                            headers,
                            status,
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
    ) -> HttpResult<bool, T> {
        let sz = yield_nb!(stack.receive(socket, buffer))?;
        self.write(&buffer[..sz]).unwrap();
        self.check_if_done()
    }

    pub fn finish<T: TcpClientStack>(self) -> HttpResult<HttpResponse, T> {
        match self {
            Self::Body {
                stream,
                content_offset,
                headers,
                status,
                total_size,
            } => {
                let body = if total_size == content_offset {
                    None
                } else {
                    Some(stream.consume()[content_offset..].into())
                };
                Ok(HttpResponse {
                    status: HttpStatus::from(status),
                    body,
                    headers,
                })
            }
            _ => Err(HttpError::EncodingFailure),
        }
    }
}
