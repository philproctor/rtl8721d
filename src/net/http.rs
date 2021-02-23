use crate::prelude::*;
use httparse::Response;

pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl HttpMethod {
    pub const fn text(&self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PUT => "PUT",
        }
    }
}

// let const x = b"GET /x HTTP/1.1\r\nHost: test\r\n\r\n";

pub struct HttpRequest {
    pub method: HttpMethod,
    pub host: Host,
    pub uri: String,
    pub body: Vec<u8>,
}

impl HttpRequest {
    pub fn new(host: Host, method: HttpMethod, uri: String, body: Vec<u8>) -> Self {
        Self {
            host,
            method,
            uri,
            body,
        }
    }

    pub async fn send(mut self) -> Result<String> {
        Network::resolve_host(&mut self.host).await;
        let req: Vec<u8> = format!(
            "{} {} HTTP/1.1\r\nHost: {}\r\n\r\n",
            self.method.text(),
            self.uri,
            self.host.host_str()
        )
        .into();

        let client = TCP::connect(self.host, 80).await.unwrap();
        // client.connect(self.host, 80).await?;
        client.write_all(&*req).await?;
        let mut buf = vec![];
        let offset = loop {
            let mut headers = [httparse::EMPTY_HEADER; 16];
            let mut rsp: Response = Response::new(&mut headers);
            let mut inbuf = client.read(1000).await?;
            buf.append(&mut inbuf);
            let parsed = rsp.parse(buf.as_slice());
            match parsed {
                Ok(httparse::Status::Complete(offset)) => {
                    debug!("Complete: {:?}", offset);
                    break offset;
                }
                Ok(httparse::Status::Partial) => continue,
                Err(e) => {
                    return Err(SystemError::ParseError(format!(
                        "Failed to parse response, {:?}",
                        e
                    )))
                }
            }
        };
        buf = buf.drain(offset..).collect();
        Ok(String::from_utf8(buf)?)
    }
}
