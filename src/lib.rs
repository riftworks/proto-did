use std::collections::HashMap;

use req::{requests::Request, uri::DIDUri, verbs::ReqVerb};

mod req;
mod error;

#[derive(Clone)]
pub struct Response {
    pub content: String
}

/// Contains the configuration of the whole server.
pub struct DIDServer {
    pub port: usize,
    pub routes: HashMap<DIDUri, fn(Request) -> dyn Future<Output = Response>>
}

impl DIDServer {
    pub fn build() -> Self {
        DIDServer {
            port: 5173,
            routes: HashMap::new()
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
        cb: fn(Request) -> dyn Future<Output = Response>
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
    pub async fn launch() {
        
    }
}
