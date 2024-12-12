#[derive(Debug)]
pub enum OrbitError {
    NegativeEccentricity(f64),
}

impl std::fmt::Display for OrbitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrbitError::NegativeEccentricity(e) => write!(f, "Eccentricity cannot be negative: {}", e),
        }
    }
}

impl std::error::Error for OrbitError {}

#[derive(Debug)]
pub enum TimeError {
    InvalidTimeScale(String),
    InvalidTimeFormat(String),
}

impl std::fmt::Display for TimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeError::InvalidTimeScale(s) => write!(f, "Invalid timescale: {}. Needs to be 'utc' or 'tdb'.", s),
            TimeError::InvalidTimeFormat(s) => write!(f, "Invalid time format: {}. Needs to be 'jd' or 'mjd'.", s),
        }
    }
}

impl std::error::Error for TimeError {}


#[derive(Debug)]
pub enum OriginError {
    InvalidOrigin(String),
}

impl std::fmt::Display for OriginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OriginError::InvalidOrigin(s) => write!(f, "Invalid origin: {}", s),
        }
    }
}

impl std::error::Error for OriginError {}