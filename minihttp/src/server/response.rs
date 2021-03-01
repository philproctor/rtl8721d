pub use crate::common::*;
use crate::util::MemoryStream;
use bare_io::Write;
use core::result::Result;

#[derive(Debug)]
pub struct OutgoingResponse<BODY: Body> {
    status_code: HttpStatus,
    headers: HttpHeaders,
    body: BODY,
}

impl<BODY: Body> OutgoingResponse<BODY> {
    pub fn content_length(&self) -> usize {
        self.body.content_length()
    }

    pub fn http_header(&self) -> Result<MemoryStream, bare_io::Error> {
        let mut res = MemoryStream::new(100);
        write!(&mut res, "HTTP/1.1 {}\r\n", self.status_code)?;
        for hdr in self.headers.iter() {
            write!(&mut res, "{}: {}\r\n", hdr.key(), hdr.value())?;
        }
        write!(&mut res, "\r\n")?;
        Ok(res)
    }

    pub fn into_stream(self) -> Result<MemoryStream, bare_io::Error> {
        let mut res = self.http_header()?;
        res.extend(self.body)?;
        Ok(res)
    }
}

impl HttpResponse {
    pub fn into_response(self) -> OutgoingResponse<MemoryStream> {
        let mut body = MemoryStream::new(100);
        body.write(self.body.unwrap().as_slice()).unwrap();
        let mut headers = HttpHeaders::new();
        headers.push(HttpHeader::new_content_type(ContentType::TextPlain));
        headers.push(HttpHeader::new_content_length(body.content_length()));
        headers.push(HttpHeader::new("Server", LIBNAME));
        headers.push(HttpHeader::new("Connection", "close"));
        OutgoingResponse {
            status_code: self.status,
            headers,
            body,
        }
    }
}
