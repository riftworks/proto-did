use std::{fmt::Display, net::Ipv4Addr, str::FromStr};
use crate::{error::DIDError, identity::DIDIdentity};
use super::verbs::ReqVerb;
use url::Url;

pub struct DIDRequest {
    pub url: Option<Url>,
    pub verb: ReqVerb,
    pub did: String,
    pub ip: Ipv4Addr,
    pub body: String
}

pub struct DIDResponse<'r> {
    pub from_req: &'r DIDRequest,
    pub with_identity: &'r DIDIdentity,
    pub content: String
}

impl FromStr for DIDRequest {
    type Err = DIDError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut header = lines.next().get_or_insert("none").split(",");

        let verb = ReqVerb::from_str(header.next().get_or_insert_default())?;
        let url = Url::from_str(header.next().get_or_insert_default());
        let mut did = header.next();
        let ip = Ipv4Addr::from_str(header.next().get_or_insert_default())
            .unwrap();

        header.next();

        let mut body = header.next();

        Ok(DIDRequest {
            verb, ip,
            url: if url.is_err() { None } else { Some(url.unwrap()) },
            did: did.get_or_insert_default().to_string(),
            body: body.get_or_insert_default().to_string()
        })
    }
}

impl<'r> Display for DIDResponse<'r> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let url_insert = if self.from_req.url.is_some() {
            format!("{},", self.from_req.url.clone().unwrap())
        } else {
            String::new()
        };

        write!(f, "{},{}{},{}\n\n{}", 
            self.from_req.verb,
            url_insert,
            self.from_req.did,
            self.from_req.ip,
            self.content
        ) 
    }
}

// TODO: Implement macros to generate request handler configs with request
// handlers from basic functions, like rocket.rs does.
