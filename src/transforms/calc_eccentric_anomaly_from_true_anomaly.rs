use crate::error::OrbitError;

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
/// let eccentricity = 0.5;
/// let true_anomaly = 0.5;
/// let result = transforms::calc_eccentric_anomaly_from_true_anomaly(eccentricity, true_anomaly);
/// ```
pub fn calc_eccentric_anomaly_from_true_anomaly(eccentricity: f64, true_anomaly: f64) -> Result<f64, OrbitError> {

    if eccentricity < 0.0 {
        return Err(OrbitError::NegativeEccentricity(eccentricity));
    }

    if eccentricity.abs() < 1.0e-10 {
        return Ok(true_anomaly);
    }

    if eccentricity < 1.0 {
        Ok(2.0 * ((1.0 - eccentricity).sqrt() * (true_anomaly / 2.0).sin()).atan2((1.0 + eccentricity).sqrt() * (true_anomaly / 2.0).cos()))
    }
    else {
        Ok(2.0 * (((eccentricity - 1.0) / (eccentricity + 1.0)).sqrt() * (true_anomaly / 2.0).tan()).atanh())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_eccentric_anomaly_from_true_anomaly() {
        let e = 0.0;
        let f = 0.0;
        match calc_eccentric_anomaly_from_true_anomaly(e, f) {
            Ok(result) => assert_eq!(result, 0.0),
            Err(_) => assert!(false),
        }

        let e = -0.1;
        let f = 0.0;
        match calc_eccentric_anomaly_from_true_anomaly(e, f) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }

    }
}