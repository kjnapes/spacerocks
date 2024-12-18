use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum TimeScale {
    #[default]
    UTC,
    TDB,
    TT, 
    TAI,
}

impl TimeScale {

    pub fn variants() -> &'static [&'static str] {
        &["UTC", "TDB", "TT", "TAI"] 
    }

    pub fn to_str(&self) -> &str {
        match self {
            TimeScale::UTC => "UTC",
            TimeScale::TDB => "TDB",
            TimeScale::TT => "TT",
            TimeScale::TAI => "TAI",
        }
    }
}

impl std::fmt::Display for TimeScale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TimeScale::UTC => write!(f, "UTC"),
            TimeScale::TDB => write!(f, "TDB"),
            TimeScale::TT => write!(f, "TT"),
            TimeScale::TAI => write!(f, "TAI"),
        }
    }
}

