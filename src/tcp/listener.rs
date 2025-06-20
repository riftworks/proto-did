use tokio::{io::{self, AsyncReadExt}, net::{TcpListener, TcpStream}};
use crate::{error::DIDError, identity::DIDIdentity};
use super::did::DIDHandler;

pub(super) trait StreamHandler<'h>: Sized {
    type Method;

    fn parse_req_header(header: &'h str) -> Vec<&'h str>;
    fn get_header_method(header: &'h str) -> Self::Method;
    async fn handle_stream(
        &mut self, identity: DIDIdentity
    ) -> Result<(), DIDError>;
    fn from_req_and_stream(
        str_req: String, sock: TcpStream
    ) -> Result<Self, DIDError>;
}

/// Will read the request and use the appropriate handler to interact with it.
///
/// All requests with the first line's items separated by "," and starting
/// with a verb will be handled by `did_stream_handler`.
/// 
/// All requests with the first line's items separated by " " and starting
/// with a HTTP method will be handled by `http_stream_handler`.
async fn redirect_to_handler(mut sock: TcpStream, identity: DIDIdentity) {
    let mut content = String::new();

    sock.read_to_string(&mut content).await.unwrap();
    if DIDHandler::get_header_method(&content).is_ok() {
        let handler = DIDHandler::from_req_and_stream(content, sock);

        if let Err(err) = handler {
            error!("{}", err.to_string());
        } else {
            handler.unwrap().handle_stream(identity).await.unwrap();
        }
    }
}

/// Will setup a TCP server that will handle both DID and HTTP requests.
pub async fn tcp_server(port: usize, identity: DIDIdentity) -> io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).await?;

    info!("Running on port {port}");
    loop {
        match listener.accept().await {
            Ok((sock, addr)) => {
                let identity = identity.clone();

                info!("{addr} connected");
                tokio::spawn(async move {
                    redirect_to_handler(sock, identity).await;
                });
            },
            Err(e) => error!("Could not get TCP stream: {e}")
        };
    }
}
