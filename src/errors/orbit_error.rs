#[derive(Debug, PartialEq)]
pub enum OrbitError {
    NegativeEccentricity(f64),
    ConvergenceFailure(f64, f64),  // (eccentricity, mean_anomaly)
}

impl std::fmt::Display for OrbitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrbitError::NegativeEccentricity(e) => write!(f, "Eccentricity cannot be negative: {}", e),
            OrbitError::ConvergenceFailure(e, m) => write!(f, "Failed to converge for eccentricity {} and mean anomaly {}", e, m),
        }
    }
}

impl std::error::Error for OrbitError {}