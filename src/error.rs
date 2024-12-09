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