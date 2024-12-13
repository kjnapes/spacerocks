use crate::errors::OrbitError;


#[derive(Debug, PartialEq)]
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbit_type_from_eccentricity() {
        let e = 0.0;
        let threshold = 1e-10;
        match OrbitType::from_eccentricity(e, threshold) {
            Ok(result) => assert_eq!(result, OrbitType::Circular),
            Err(_) => assert!(false),
        }

        let e = -0.1;
        match OrbitType::from_eccentricity(e, threshold) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }

        let e = 0.5;
        match OrbitType::from_eccentricity(e, threshold) {
            Ok(result) => assert_eq!(result, OrbitType::Elliptical),
            Err(_) => assert!(false),
        }

        let e = 1.0;
        match OrbitType::from_eccentricity(e, threshold) {
            Ok(result) => assert_eq!(result, OrbitType::Parabolic),
            Err(_) => assert!(false),
        }

        let e = 1.1;
        match OrbitType::from_eccentricity(e, threshold) {
            Ok(result) => assert_eq!(result, OrbitType::Hyperbolic),
            Err(_) => assert!(false),
        }
    }
}