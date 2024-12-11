use crate::error::OrbitError;
use crate::orbit_type::OrbitType;

/// Calculate the conic anomaly (eccentric, parabolic, or hyperbolic) from the true anomaly.
///
/// # Arguments
///
/// * `e` - Eccentricity of the orbit.
/// * `eccentric_anomaly` - Eccentric anomaly of the orbit in radians.
///
/// # Returns
///
/// * `Result<f64, OrbitError>` - The mean anomaly in radians.
///
/// # Example
///
/// ```
/// use spacerocks::transforms;
/// let e = 0.5;
/// let conic_anomaly = 0.5;
/// let result = transforms::calc_mean_anomaly_from_conic_anomaly(e, conic_anomaly);
/// ```
#[allow(non_snake_case)]
pub fn calc_mean_anomaly_from_conic_anomaly(e: f64, conic_anomaly: f64) -> Result<f64, OrbitError> {
    
    let orbit_type = OrbitType::from_eccentricity(e, 1e-10)?; // returns OrbitError error if e < 0.0
    match orbit_type {
        OrbitType::Circular => Ok(conic_anomaly),
        OrbitType::Elliptical => Ok(conic_anomaly - e * conic_anomaly.sin()),
        OrbitType::Parabolic => Ok(conic_anomaly - conic_anomaly.powi(3) / 3.0),
        OrbitType::Hyperbolic => Ok(e * conic_anomaly.sinh() - conic_anomaly),
        OrbitType::Radial => unreachable!(),
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_mean_anomaly_from_conic_anomaly() {
        let e = 0.0;
        let conic_anomaly = 0.0;
        match calc_mean_anomaly_from_conic_anomaly(e, conic_anomaly) {
            Ok(result) => assert_eq!(result, 0.0),
            Err(_) => assert!(false),
        }

        let e = -0.1;
        let conic_anomaly = 0.0;
        match calc_mean_anomaly_from_conic_anomaly(e, conic_anomaly) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }
}