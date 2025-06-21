use tokio::{io::AsyncReadExt, net::TcpStream};

use super::{did::DIDHandler, listener::StreamHandler};

/// Will read a stream until the end of the expected size by looking either at
/// the size positional argument of DID requests or the Content-Length header
/// of HTTP requests.
///
/// To properly work, it will read byte by byte until new lines. If the current
/// new line is a "Content-Type" header or a DID header, the size is retrieved
/// and the entire request is read.
/// TODO: Implement Content-Type lookup
pub async fn read_expected_size(mut sock: TcpStream) -> String {
    let mut content = String::new();
    let mut buffer: Vec<u8> = vec![0, 1];
    let mut req_size: usize = 0;

    sock.readable().await.unwrap();
    while req_size == 0 {
        sock.read_exact(&mut buffer).await.unwrap();
        content.push(buffer[0] as char);
        buffer.clear();
        if content.ends_with("\n") {
            let did_header = DIDHandler::parse_req_header(&content);

            if did_header.len() == 5 {
                req_size = usize::from_str_radix(
                    did_header.get(4).unwrap(), 10
                ).unwrap();
            }
        }
    }
    buffer = Vec::with_capacity(req_size - content.len());
    sock.read_exact(&mut buffer).await.unwrap();
    content.push_str(&String::from_utf8(buffer).unwrap());

    content
}
