use crate::error::OrbitError;
use crate::orbit_type::OrbitType;

use nalgebrs::Vector3;

struct KeplerOrbit {
    pub specific_energy: f64,
    pub h: f64,
    pub hz: f64,
}

/// Calculate a Keplerian orbit given a state vector.
///
/// # Arguments
///
/// * `position` - Position vector 
/// * `velocity` - Velocity vector
/// * `mu` - Gravitational parameter of the central body
///
/// # Returns
///
/// * `Result<f64, OrbitError>` - The mean anomaly in radians.
pub fn calc_kep_from_state(position: Vector3, velocity: Vector3, mu: f64) -> Result<KeplerOrbit, OrbitError> {

    let rsq = position.norm_squared();
    let vsq = velocity.norm_squared();
    let vr = position.dot(&velocity) / position.norm();

    let specific_energy = vsq / 2.0 - mu / position.norm();
    let hvec = position.cross(&velocity);
    let h = hvec.norm();

    // calculate the eccentricity vector
    let evec = (velocity * (vsq - mu / position.norm()) - position * vr).normalize();
    let e = evec.norm();

    let sin_true_anomaly = h * vr / mu;
    let cos_true_anomaly = (h * h / (mu * r) - 1) / e;
    let true_anomaly = sin_true_anomaly.atan2(cos_true_anomaly);

    

}
