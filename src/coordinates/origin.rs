use serde::{Serialize, Deserialize};
use crate::errors::OriginError;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum Origin {
    SUN,
    #[default]
    SSB,
    Custom {name: String, mu: f64},
}

impl Origin {

    pub fn new_custom(mu: f64, name: &str) -> Origin {
        Origin::Custom { mu, name: name.to_string() }
    }

    pub fn from_str(s: &str) -> Result<Origin, OriginError> {
        match s.to_uppercase().as_str() {
            "SUN" => Ok(Origin::SUN),
            "SSB" => Ok(Origin::SSB),
            _ => Err(OriginError::InvalidOrigin(s.to_string())),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Origin::SUN => "SUN",
            Origin::SSB => "SSB",
            Origin::Custom { name, .. } => name,
        }
    }

    pub fn ssb() -> Origin {
        Origin::SSB
    }

    pub fn sun() -> Origin {
        Origin::SUN
    }

    pub fn mu(&self) -> f64 {
        match self {
            Origin::SUN => 0.000_295_912_208_284_119_5,
            Origin::SSB => 0.00029630927493457475,
            Origin::Custom { mu, .. } => *mu,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Origin::SUN => "SUN",
            Origin::SSB => "SSB",
            Origin::Custom { name, .. } => name,
        }
    }

    pub fn to_string(&self) -> String {
        self.name().to_string()
    }
}



impl std::fmt::Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}