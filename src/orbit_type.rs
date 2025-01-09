use crate::errors::OrbitError;


#[derive(Debug, PartialEq, Clone)]
pub enum OrbitType {
    Hyperbolic,
    Parabolic,
    Elliptical,
    Circular,
    Radial,
}

impl OrbitType {
    pub fn from_eccentricity(e: f64, threshold: f64) -> Result<OrbitType, OrbitError> {
        match e {
            e if e < 0.0 => Err(OrbitError::NegativeEccentricity(e)),
            e if e < threshold => Ok(OrbitType::Circular),
            e if e < 1.0 => Ok(OrbitType::Elliptical),
            e if (e - 1.0).abs() < threshold => Ok(OrbitType::Parabolic),
            _ => Ok(OrbitType::Hyperbolic),
        }
    }
}
