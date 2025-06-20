use std::{fmt::Display, str::FromStr};
use crate::error::DIDError;

/// Implementation of request verbs
#[derive(Eq, Hash, PartialEq)]
pub enum ReqVerb {
    /// PREFLIGHT
    Preflight,
    /// WHERE?
    Where,
    /// WHERE!
    WhereStorage,
    /// #DATA
    HashData,
    /// DATA
    Data
}

impl Display for ReqVerb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode = match self {
            Self::Preflight => "PREFLIGHT",
            Self::Where => "WHERE?",
            Self::WhereStorage => "WHERE!",
            Self::HashData => "#DATA",
            Self::Data => "DATA"
        };

        write!(f, "{}", mode)
    }
}

impl FromStr for ReqVerb {
    type Err = DIDError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PREFLIGHT" => Ok(Self::Preflight),
            "WHERE?" => Ok(Self::Where),
            "WHERE!" => Ok(Self::WhereStorage),
            "#DATA" => Ok(Self::HashData),
            "DATA" => Ok(Self::Data),
            _ => Err(DIDError {
                source: "ReqVerbs::from_str".to_string(),
                reason: "unknown verb".to_string()
            })
        }
    }
}
