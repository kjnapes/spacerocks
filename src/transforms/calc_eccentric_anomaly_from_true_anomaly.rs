/// Calculate the eccentric anomaly from the true anomaly.
///
/// # Arguments
///
/// * `eccentricity` - Eccentricity of the orbit.
/// * `true_anomaly` - True anomaly of the orbit in radians.
///
/// # Returns
///
/// * `Option<f64>` - The eccentric anomaly in radians.
///
/// # Example
///
/// ```
/// use spacerocks::transforms;
/// let eccentricity = 0.5;
/// let true_anomaly = 0.5;
/// let result = transforms::calc_eccentric_anomaly_from_true_anomaly(eccentricity, true_anomaly);
/// ```
pub fn calc_eccentric_anomaly_from_true_anomaly(eccentricity: f64, true_anomaly: f64) -> f64 {

    if eccentricity.abs() < 1.0e-10 {
        return true_anomaly;
    }

    if eccentricity < 1.0 {
        2.0 * ((1.0 - eccentricity).sqrt() * (true_anomaly / 2.0).sin()).atan2((1.0 + eccentricity).sqrt() * (true_anomaly / 2.0).cos())
    }
    else {
        2.0 * (((eccentricity - 1.0) / (eccentricity + 1.0)).sqrt() * (true_anomaly / 2.0).tan()).atanh()
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_eccentric_anomaly_from_true_anomaly() {
        let e = 0.0;
        let f = 0.0;
        let result = calc_eccentric_anomaly_from_true_anomaly(e, f);
        assert_eq!(result, 0.0);
    }
}



// let mut E;

    // if e < 1.0 {
    //     let E = 2.0 * ((1.0 - e).sqrt() * (f / 2.0).sin()).atan2((1.0 + e).sqrt() * (f / 2.0).cos());
    // }

    // else {
    //     // let cta = f.cos();
    //     // E = ((cta + e) / (1.0 + e * cta)).acosh();
    //     // if f < 0.0 {
    //     //     E *= -1.0;
    //     // }

    //     let E = 2.0 * (((e - 1.0) / (e + 1.0)).sqrt() * (f / 2.0).tan()).atanh();
    // }
    // return E;