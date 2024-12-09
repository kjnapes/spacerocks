use crate::error::OrbitError;

/// Calculate the eccentric anomaly from the true anomaly.
///
/// # Arguments
///
/// * `eccentricity` - Eccentricity of the orbit.
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
/// let eccentricity = 0.5;
/// let eccentric_anomaly = 0.5;
/// let result = transforms::calc_mean_anomaly_from_eccentric_anomaly(eccentricity, eccentric_anomaly);
/// ```
#[allow(non_snake_case)]
pub fn calc_mean_anomaly_from_eccentric_anomaly(eccentricity: f64, eccentric_anomaly: f64) -> Result<f64, OrbitError> {
    
    if eccentricity < 0.0 {
        return Err(OrbitError::NegativeEccentricity(eccentricity));
    }

    if eccentricity < 1.0 {
        Ok(eccentric_anomaly - eccentricity * eccentric_anomaly.sin())
    }
    else {
        Ok(eccentricity * eccentric_anomaly.sinh() - eccentric_anomaly)
    }

}