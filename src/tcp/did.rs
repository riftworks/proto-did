use std::str::FromStr;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use crate::{error::DIDError, identity::DIDIdentity, req::{reqres::{DIDRequest, DIDResponse}, verbs::ReqVerb}};
use super::listener::StreamHandler;

pub(super) struct DIDHandler {
    latest_req: DIDRequest,
    sock: TcpStream
}

impl DIDHandler {
    fn process_latest_request<'p>(
        req: &'p DIDRequest,
        identity: &'p DIDIdentity
    ) -> DIDResponse<'p> {
        DIDResponse {
            from_req: req,
            with_identity: identity,
            content: "OK".into()
        }
    }
}

impl<'h> StreamHandler<'h> for DIDHandler {
    type Method = Result<ReqVerb, DIDError>;

    fn parse_req_header(header: &'h str) -> Vec<&'h str> {
        header.split(",").into_iter().collect()
    }

    fn get_header_method(header: &'h str) -> Self::Method {
        let items = DIDHandler::parse_req_header(header);

        ReqVerb::from_str(items[0])
    }

    /// When dealing with a DID TCP stream
    async fn handle_stream(
        &mut self, identity: DIDIdentity
    ) -> Result<(), DIDError> {
        let socket = &mut self.sock;
        let mut req_str = String::new();
        let res = DIDHandler::process_latest_request(
            &self.latest_req, &identity
        );

        socket.writable().await.unwrap();
        socket.write_all(res.to_string().as_bytes()).await.unwrap();

        loop {
            socket.readable().await.unwrap();
            socket.read_to_string(&mut req_str).await.unwrap();

            if req_str.is_empty() {
                continue
            }

            if let Ok(req) = DIDRequest::from_str(&req_str) {
                if req.ip != self.latest_req.ip {
                    error!("{}: {}", self.latest_req.ip, "IP mismatch");
                } else {
                    let res = DIDHandler::process_latest_request(
                        &self.latest_req, &identity
                    );

                    socket.writable().await.unwrap();
                    socket.write_all(res.to_string().as_bytes()).await.unwrap();
                    self.latest_req = req;
                }
            } else {
                error!(
                    "{}: {}",
                    self.latest_req.ip,
                    "Failed to parse request content"
                );
            }
        }
    }

    fn from_req_and_stream(
        str_req: String,
        sock: TcpStream
    ) -> Result<Self, DIDError> {
        if let Ok(latest_req) = DIDRequest::from_str(&str_req) {
            return Ok(Self { latest_req, sock });
        }

        Err(DIDError {
            source: "DIDHandler::handle_stream".into(),
            reason: "Failed to parse request content".into()
        })
    }
}
