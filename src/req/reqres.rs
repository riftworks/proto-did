use std::{fmt::Display, net::Ipv4Addr, str::FromStr};
use crate::{error::DIDError, identity::DIDIdentity};
use super::verbs::ReqVerb;
use url::Url;

pub struct DIDRequest {
    pub url: Option<Url>,
    pub verb: ReqVerb,
    pub did: String,
    pub req_size: usize,
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
        let size = usize::from_str(header.next().get_or_insert("0")).unwrap();

        header.next();

        let mut body = header.next();

        Ok(DIDRequest {
            verb, ip,
            req_size: size,
            url: if url.is_err() { None } else { Some(url.unwrap()) },
            did: did.get_or_insert_default().to_string(),
            body: body.get_or_insert_default().to_string()
        })
    }
}

impl Display for DIDRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let url_insert = if self.url.is_some() {
            format!("{},", self.url.clone().unwrap())
        } else {
            String::new()
        };

        let size = self.verb.to_string().len() +
            self.did.to_string().len() +
            self.ip.to_string().len() +
            self.body.len() +
            url_insert.len() + 5;

        let size = size + size.to_string().len();

        write!(f, "{},{}{},{},{}\n\n{}", 
            self.verb,
            url_insert,
            self.did,
            self.ip,
            size,
            self.body
        ) 
    }
}

impl<'r> Display for DIDResponse<'r> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let url_insert = if self.from_req.url.is_some() {
            format!("{},", self.from_req.url.clone().unwrap())
        } else {
            String::new()
        };

        let size = self.from_req.verb.to_string().len() +
            self.from_req.did.to_string().len() +
            self.from_req.ip.to_string().len() +
            self.content.len() +
            url_insert.len() + 5;

        let size = size + size.to_string().len();

        write!(f, "{},{}{},{},{}\n\n{}", 
            self.from_req.verb,
            url_insert,
            self.from_req.did,
            self.from_req.ip,
            size,
            self.content
        ) 
    }
}

// TODO: Implement macros to generate request handler configs with request
// handlers from basic functions, like rocket.rs does.
