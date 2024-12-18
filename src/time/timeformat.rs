use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum TimeFormat {
    #[default]
    JD,
    MJD,
}

impl TimeFormat {

    pub fn variants() -> &'static [&'static str] {
        &["JD", "MJD"]
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "JD" => Some(TimeFormat::JD),
            "MJD" => Some(TimeFormat::MJD),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            TimeFormat::JD => "JD",
            TimeFormat::MJD => "MJD",
        }
    }
}

impl std::fmt::Display for TimeFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TimeFormat::JD => write!(f, "JD"),
            TimeFormat::MJD => write!(f, "MJD"),
        }
    }
}

