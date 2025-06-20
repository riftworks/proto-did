use super::verbs::ReqVerb;
use url::Url;

pub struct Request {
    pub url: Url,
    pub verb: ReqVerb,
    pub did: String
}

// TODO: Implement macros to generate request handler configs with request
// handlers from basic functions, like rocket.rs does.
