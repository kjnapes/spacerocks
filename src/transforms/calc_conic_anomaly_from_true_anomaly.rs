use crate::error::OrbitError;
use crate::orbit_type::OrbitType;

/// Calculate the eccentric anomaly from the true anomaly.
///
/// # Arguments
///
/// * `eccentricity` - Eccentricity of the orbit.
/// * `true_anomaly` - True anomaly of the orbit in radians.
///
/// # Returns
///
/// * `Result<f64, OrbitError>` - The eccentric anomaly in radians.
///
/// # Example
///
/// ```
/// use spacerocks::transforms;
/// let e = 0.5;
/// let true_anomaly = 0.5;
/// let result = transforms::calc_conic_anomaly_from_true_anomaly(e, true_anomaly);
/// ```
pub fn calc_conic_anomaly_from_true_anomaly(e: f64, true_anomaly: f64) -> Result<f64, OrbitError> {

    let orbit_type = OrbitType::from_eccentricity(e, 1e-10)?; // returns OrbitError error if e < 0.0
    match orbit_type {
        OrbitType::Circular => Ok(true_anomaly),
        OrbitType::Elliptical => Ok(2.0 * ((1.0 - e).sqrt() * (true_anomaly / 2.0).sin()).atan2((1.0 + e).sqrt() * (true_anomaly / 2.0).cos())),
        OrbitType::Parabolic => Ok((true_anomaly / 2.0).tan()),
        OrbitType::Hyperbolic => Ok(2.0 * (((e - 1.0) / (e + 1.0)).sqrt() * (true_anomaly / 2.0).tan()).atanh()),
        OrbitType::Radial => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_conic_anomaly_from_true_anomaly() {
        let e = 0.0;
        let f = 0.0;
        match calc_conic_anomaly_from_true_anomaly(e, f) {
            Ok(result) => assert_eq!(result, 0.0),
            Err(_) => assert!(false),
        }

        let e = -0.1;
        let f = 0.0;
        match calc_conic_anomaly_from_true_anomaly(e, f) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }

    }
}