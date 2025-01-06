use crate::orbit_type::OrbitType;
use crate::errors::OrbitError;

pub fn calc_true_anomaly_from_conic_anomaly(e: f64, conic_anomaly: f64) -> Result<f64, OrbitError> {

    let orbit_type = OrbitType::from_eccentricity(e, 1e-10)?;
    match orbit_type {
        OrbitType::Circular => Ok(true_anomaly_from_conic_anomaly_circular(e, conic_anomaly)),
        OrbitType::Elliptical => Ok(true_anomaly_from_conic_anomaly_elliptical(e, conic_anomaly)),
        OrbitType::Parabolic => Ok(true_anomaly_from_conic_anomaly_parabolic(e, conic_anomaly)),
        OrbitType::Hyperbolic => Ok(true_anomaly_from_conic_anomaly_hyperbolic(e, conic_anomaly)),
        OrbitType::Radial => unreachable!(),
    }
}

fn true_anomaly_from_conic_anomaly_circular(_e: f64, conic_anomaly: f64) -> f64 {
    return conic_anomaly;
}

fn true_anomaly_from_conic_anomaly_elliptical(e: f64, conic_anomaly: f64) -> f64 {
    return 2.0 * ((1.0 + e).sqrt() * (conic_anomaly / 2.0).sin()).atan2((1.0 - e).sqrt() * (conic_anomaly / 2.0).cos());
}

fn true_anomaly_from_conic_anomaly_parabolic(_e: f64, conic_anomaly: f64) -> f64 {
    return 2.0 * conic_anomaly.atan2(1.0);
}

fn true_anomaly_from_conic_anomaly_hyperbolic(e: f64, conic_anomaly: f64) -> f64 {
    return 2.0 * (((e + 1.0) / (e - 1.0)).sqrt() * (conic_anomaly / 2.0).tanh()).atan2(1.0);
}