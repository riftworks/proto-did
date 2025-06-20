use std::{net::Ipv4Addr, str::FromStr};

use crate::error::DIDError;
use super::verbs::ReqVerb;
use url::Url;

pub struct DIDRequest {
    pub url: Option<Url>,
    pub verb: ReqVerb,
    pub did: String,
    pub ip: Ipv4Addr,
    pub body: String
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

// TODO: Implement macros to generate request handler configs with request
// handlers from basic functions, like rocket.rs does.
