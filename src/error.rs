use std::{error::Error, fmt::{Debug, Display}};

pub struct DIDError {
    pub source: String,
    pub reason: String
}

impl Display for DIDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.source, self.reason)
    }
}

impl Debug for DIDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DIDError")
            .field("source", &self.source)
            .field("reason", &self.reason)
            .finish()
    }
}

impl Error for DIDError {}
