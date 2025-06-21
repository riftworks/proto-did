#[macro_use] extern crate log;

use std::collections::HashMap;
use identity::DIDIdentity;
use req::{reqres::{DIDRequest, DIDResponse}, uri::DIDUri, verbs::ReqVerb};
use tcp::listener::tcp_server;

pub mod cli;
mod tcp;
mod req;
mod error;
mod identity;

/// Contains the configuration of the whole server.
pub struct DIDServer<'s> {
    pub port: usize,
    pub routes: HashMap<
        DIDUri, fn(DIDRequest) -> dyn Future<Output = DIDResponse<'s>>>,
    pub identity: DIDIdentity,
    pub http_enabled: bool,
    pub did_enabled: bool
}

impl<'s> DIDServer<'s> {
    pub fn build() -> Self {
        env_logger::init();
        DIDServer {
            port: 5173,
            routes: HashMap::new(),
            identity: DIDIdentity {
                did: "imapotato".to_string()
            },
            http_enabled: true,
            did_enabled: true
        }
    }


    pub fn set_port(&mut self, port: usize) -> &mut Self {
        self.port = port;
        self
    }

    pub fn add_route<F>(
        &mut self,
        verb: ReqVerb,
        path: &str,
        cb: fn(DIDRequest) -> dyn Future<Output = DIDResponse<'s>>
    ) -> &mut Self {
        self.routes.insert(DIDUri {
            url: None,
            path: Some(path.to_string()),
            verb
        }, cb);
        self
    }

    /// Launche a socket listener on `self.port`. This
    /// function must be called after initializing everything you need in your
    /// app.
    ///
    /// Usage:
    /// ```rust
    ///  #[tokio::main]
    ///  async fn main() {
    ///     DIDServer::build()
    ///         .set_port(3000)
    ///         .add_route(ReqVeb::Where, "/", index)
    ///         .launch()
    ///         .await
    ///  }
    /// ```
    pub async fn launch(&self) {
        tcp_server(self.port, self.identity.clone())
            .await
            .expect("TcpServer error!");
    }
}
