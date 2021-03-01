use super::*;
use embedded_nal::SocketAddr;

mod request;
mod response;

pub use request::OutgoingRequest;
pub use response::IncomingResponse;

impl<T: ClientStack + ServerStack> Http<T> {
    pub async fn send_request(&self, request: OutgoingRequest) -> HttpResult<String, T> {
        let mut stack = self.stack.clone();
        let remote = stack
            .get_host_by_name(&request.host, AddrType::IPv4)
            .map_err(|_| HttpError::UnknownHost)?;
        let mut socket = stack.socket()?;
        let buf = request.consume();
        yield_nb!(stack.connect(&mut socket, SocketAddr::new(remote, 80)))?;
        yield_nb!(stack.send(&mut socket, &buf))?;

        let mut inbuf = [0u8; 32];
        let mut response_stream = IncomingResponse::new();
        while !response_stream
            .read_from_stream(&mut stack, &mut socket, &mut inbuf)
            .await?
        {}
        let rsp_obj = response_stream.finish::<T>()?;
        Ok(String::from_utf8(rsp_obj.body.unwrap_or_default())
            .map_err(|_| HttpError::EncodingFailure)?)
    }
}
