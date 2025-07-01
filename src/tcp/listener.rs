use std::{
    time::{SystemTime, UNIX_EPOCH}};
use rlimit::{getrlimit, Resource};
use tokio::{io,
    net::{TcpListener, TcpStream},
    sync::oneshot::{self, Receiver, Sender}};
use crate::{
    error::DIDError,
    identity::DIDIdentity,
    tcp::stream::read_expected_size};
use super::did::DIDHandler;

pub(super) struct SockCacheEntry {
    channel_sender: Sender<u8>,
    created_at: u128
}

pub(super) trait StreamHandler<'h>: Sized {
    type Method;

    fn parse_req_header(header: &'h str) -> Vec<&'h str>;
    fn get_header_method(header: &'h str) -> Self::Method;
    async fn handle_stream(
        &mut self, identity: DIDIdentity, rx: Receiver<u8>
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
async fn redirect_to_handler(
    mut sock: TcpStream,
    identity: DIDIdentity,
    rx: Receiver<u8>
) {
    let content = read_expected_size(&mut sock).await;
    if DIDHandler::get_header_method(&content).is_ok() {
        let handler = DIDHandler::from_req_and_stream(content, sock);

        if let Err(err) = handler {
            error!("{}", err.to_string());
        } else {
            handler.unwrap().handle_stream(identity, rx).await.unwrap();
        }
    }
}

/// Will setup a TCP server that will handle both DID and HTTP requests.
pub async fn tcp_server(port: usize, identity: DIDIdentity) -> io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).await?;
    // The sock_list is used to store all active TCP connections and manage
    // them according to the current node needs. As defined in the 
    // documentation starting at line 194: Session caching.
    let mut sock_list: Vec<SockCacheEntry> = vec![];

    let (max_files, _) = getrlimit(Resource::NOFILE)
        .expect("Failed to get NOFILE");

    // To avoid issues with TCP connections failing to open, we get the maximum
    // number of files (incl. sockets) the app is allowed to get open and we
    // keep being at `max - 1` to prevent the system from denying new 
    // connections.

    info!("Running on port {port}");
    loop {
        match listener.accept().await {
            Ok((sock, addr)) => {
                if sock_list.len() == (max_files - 1).try_into().unwrap() {
                    let mut oldest_timestamp = 0;
                    
                    sock_list.iter().for_each(|sock| {
                        oldest_timestamp = sock.created_at;
                    });
                    
                    let sock_pos = sock_list.iter()
                        .position(|s| { s.created_at == oldest_timestamp })
                        .unwrap();
                    let sock = sock_list.remove(sock_pos);

                    sock.channel_sender.send(0).unwrap();
                }
                
                let (tx, rx): (Sender<u8>, Receiver<u8>) = oneshot::channel();
                let cache_instance = SockCacheEntry {
                    channel_sender: tx,
                    created_at: SystemTime::now().duration_since(UNIX_EPOCH)
                        .unwrap().as_millis()
                };
                let identity = identity.clone();

                sock_list.push(cache_instance);

                info!("{addr} connected");
                tokio::spawn(async move {
                    redirect_to_handler(sock, identity, rx).await;
                });
            },
            Err(e) => error!("Could not get TCP stream: {e}")
        };
    }
}

