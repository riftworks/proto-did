use std::fmt::Display;
use url::Url;
use super::verbs::ReqVerb;

/// Describes a DID URI and a verb, it can be used with an absolute URI (with
/// protocol) therefore Some(url), or a relative one (absolute path) with
/// Some(path)
#[derive(Eq, Hash, PartialEq)]
pub struct DIDUri {
    pub url: Option<Url>,
    pub path: Option<String>,
    pub verb: ReqVerb
}

impl Display for DIDUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.verb, self.verb)
    }
}
