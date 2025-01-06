use spacerocks::OrbitType;
use spacerocks::SpaceRock;
use spacerocks::Observer;
use spacerocks::Time;
use spacerocks::transforms::kep_from_xyz::calc_kep_from_state;
use spacerocks::transforms::calc_conic_anomaly_from_mean_anomaly;
use spacerocks::transforms::calc_conic_anomaly_from_true_anomaly;
use spacerocks::transforms::calc_mean_anomaly_from_conic_anomaly;
use spacerocks::transforms::correct_for_ltt;
use spacerocks::constants::SPEED_OF_LIGHT;

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    use nalgebra::Vector3;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_orbit_type_classification() {
        let threshold = 1e-10;
        
        // Test circular orbit
        let e = 0.0;
        match OrbitType::from_eccentricity(e, threshold) {
            Ok(result) => assert_eq!(result, OrbitType::Circular),
            Err(_) => assert!(false, "Failed to classify circular orbit"),
        }

        // Test elliptical orbit
        let e = 0.5;
        match OrbitType::from_eccentricity(e, threshold) {
            Ok(result) => assert_eq!(result, OrbitType::Elliptical),
            Err(_) => assert!(false, "Failed to classify elliptical orbit"),
        }

        // Test parabolic orbit
        let e = 1.0;
        match OrbitType::from_eccentricity(e, threshold) {
            Ok(result) => assert_eq!(result, OrbitType::Parabolic),
            Err(_) => assert!(false, "Failed to classify parabolic orbit"),
        }

        // Test hyperbolic orbit
        let e = 1.5;
        match OrbitType::from_eccentricity(e, threshold) {
            Ok(result) => assert_eq!(result, OrbitType::Hyperbolic),
            Err(_) => assert!(false, "Failed to classify hyperbolic orbit"),
        }

        // Test invalid eccentricity
        let e = -0.1;
        match OrbitType::from_eccentricity(e, threshold) {
            Ok(_) => assert!(false, "Should reject negative eccentricity"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn test_calc_eccentric_anomaly_from_mean_anomaly() {
        let e = 0.0;
        let mean_anomaly = 0.0;
        match calc_conic_anomaly_from_mean_anomaly(e, mean_anomaly) {
            Ok(result) => assert_eq!(result, 0.0),
            Err(_) => assert!(false),
        }

        let e = 0.5;
        let mean_anomaly = 0.5;
        match calc_conic_anomaly_from_mean_anomaly(e, mean_anomaly) {
            Ok(result) => assert_eq!(result, 0.5),
            Err(_) => assert!(false),
        }

        let e = -0.1;
        let mean_anomaly = 0.0;
        match calc_conic_anomaly_from_mean_anomaly(e, mean_anomaly) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }

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

    #[test]
    fn test_light_time_correction() {
        let rock = SpaceRock::from_xyz(
            "test_rock",
            1.0, 0.0, 0.0,  // 1 AU from origin
            0.0, 29.78, 0.0,  // Earth-like orbital velocity
            Time::now(),
            "J2000",
            "SSB"
        ).unwrap();

        let observer_position = Vector3::new(0.0, 0.0, 0.0);  // Observer at origin
        let observer_velocity = Vector3::new(0.0, 0.0, 0.0);  // Stationary observer
        let observer = Observer::new(observer_position, observer_velocity);

        let corrected = correct_for_ltt(&rock, &observer);

        // Verify light time effect
        let distance = (corrected.position - observer.position()).norm();
        let light_time = distance / SPEED_OF_LIGHT;
        assert!(light_time > 0.0);

        // Verify position difference
        let position_difference = (corrected.position - rock.position).norm();
        assert!(position_difference > 0.0);
    }

    #[test]
    fn test_state_to_keplerian() {
        // Test circular orbit
        let position = Vector3::new(1.0, 0.0, 0.0);
        let velocity = Vector3::new(0.0, 1.0, 0.0);
        let mu = 1.0;
        
        match calc_kep_from_state(position, velocity, mu) {
            Ok(orbit) => {
                assert!(orbit.h > 0.0);  // Angular momentum should be positive
                assert!(orbit.specific_energy < 0.0);  // Bound orbit
            },
            Err(_) => assert!(false, "Failed to calculate Keplerian elements"),
        }

        // Test zero position vector (should fail)
        let position = Vector3::new(0.0, 0.0, 0.0);
        let velocity = Vector3::new(0.0, 1.0, 0.0);
        match calc_kep_from_state(position, velocity, mu) {
            Ok(_) => assert!(false, "Should reject zero position vector"),
            Err(_) => assert!(true),
        }
    }
}