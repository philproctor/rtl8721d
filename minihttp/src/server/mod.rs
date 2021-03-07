use super::*;
use core::result::Result;

mod handler;
mod request;
mod response;

pub use handler::*;
pub use request::*;
pub use response::*;

impl<T: ClientStack + ServerStack> Http<T> {
    pub async fn server<Q: HttpHandler>(
        &self,
    ) -> Result<(), HttpError<<T as TcpClientStack>::Error>> {
        let mut s = self.stack.clone();
        let mut sock = s.socket()?;
        s.bind(&mut sock, 8080)?;
        s.listen(&mut sock)?;
        loop {
            let (mut c_sock, _) = yield_nb!(s.accept(&mut sock))?;
            let mut request_stream = IncomingRequest::new();
            let mut inbuf = [0u8; 16];
            while !request_stream
                .read_from_stream(&mut s, &mut c_sock, &mut inbuf)
                .await?
            {}
            let req = request_stream.finish::<T>()?;
            let res = Q::handle(req).unwrap().await;
            let data = res.into_response();
            let stream = data.into_stream().unwrap();
            yield_nb!(s.send(&mut c_sock, stream.buf_ref()))?;
            s.close(c_sock)?;
        }
    }
}
