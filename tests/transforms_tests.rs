use spacerocks::OrbitType;
use spacerocks::SpaceRock;
use spacerocks::Observer;
use spacerocks::Time;
use spacerocks::transforms::kep_from_xyz::calc_kep_from_state;
use spacerocks::transforms::calc_conic_anomaly_from_mean_anomaly;
use spacerocks::transforms::calc_conic_anomaly_from_true_anomaly;
use spacerocks::transforms::calc_mean_anomaly_from_conic_anomaly;
use spacerocks::transforms::calc_true_anomaly_from_conic_anomaly;
use spacerocks::transforms::calc_true_anomaly_from_mean_anomaly;
use spacerocks::transforms::calc_xyz_from_kepM;
use spacerocks::transforms::correct_for_ltt;
use spacerocks::constants::SPEED_OF_LIGHT;
use spacerocks::errors::OrbitError;
use spacerocks::constants::MU_BARY;

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    use nalgebra::Vector3;
    

    const EPSILON: f64 = 1e-10;

    /// Orbit Classification Tests
    #[test]
    fn test_orbit_type_classification() {
        let threshold = 1e-10;
        
        // Test circular orbit
        let e = 0.0;
        match OrbitType::from_eccentricity(e, EPSILON) {
            Ok(result) => assert_eq!(result, OrbitType::Circular),
            Err(_) => assert!(false, "Failed to classify circular orbit"),
        }

        // Test elliptical orbit
        let e = 0.5;
        match OrbitType::from_eccentricity(e, EPSILON) {
            Ok(result) => assert_eq!(result, OrbitType::Elliptical),
            Err(_) => assert!(false, "Failed to classify elliptical orbit"),
        }

        // Test parabolic orbit
        let e = 1.0;
        match OrbitType::from_eccentricity(e, EPSILON) {
            Ok(result) => assert_eq!(result, OrbitType::Parabolic),
            Err(_) => assert!(false, "Failed to classify parabolic orbit"),
        }

        // Test hyperbolic orbit
        let e = 1.5;
        match OrbitType::from_eccentricity(e, EPSILON) {
            Ok(result) => assert_eq!(result, OrbitType::Hyperbolic),
            Err(_) => assert!(false, "Failed to classify hyperbolic orbit"),
        }

        // Test invalid eccentricity
        let e = -0.1;
        match OrbitType::from_eccentricity(e, EPSILON) {
            Ok(_) => assert!(false, "Should reject negative eccentricity"),
            Err(err) => assert_eq!(err, OrbitError::NegativeEccentricity(e)),
        }
    }

    /// Testing conic anomaly from mean anomaly

    #[test]
    fn test_calc_conic_anomaly_from_mean_anomaly() {

        // Test circular orbits (e = 0)

        let e = 0.0;
        let test_anomalies = vec![
            0.0, 
            std::f64::consts::PI / 4.0,
            std::f64::consts::PI / 2.0,
            std::f64::consts::PI,
            3.0 * std::f64::consts::PI / 2.0,
            2.0 * std::f64::consts::PI
        ];
        
        for &M in &test_anomalies {
            match calc_conic_anomaly_from_mean_anomaly(e, M) {
                Ok(result) => assert!((result - M).abs() < EPSILON, 
                    "Circular orbit: for M = {}, expected {}, got {}", M, M, result),
                Err(_) => assert!(false, "Circular orbit should not fail for e = {}, M = {}", e, M),
            }
        }
    
        // Test elliptical orbits (0 < e < 1)

        let elliptical_es = vec![0.1, 0.5, 0.7, 0.9];
        for &e in &elliptical_es {
            for &M in &test_anomalies {
                match calc_conic_anomaly_from_mean_anomaly(e, M) {
                    Ok(E) => {
                        // Check Kepler's equation: M = E - e*sin(E)
                        let computed_M = E - e * E.sin();
                        assert!((computed_M - M).abs() < EPSILON, 
                            "Elliptical orbit: for e = {}, M = {}, got E = {}, which gives M = {}", 
                            e, M, E, computed_M);
                    },
                    Err(_) => assert!(false, "Elliptical orbit should not fail for e = {}, M = {}", e, M),
                }
            }
        }
    
        // Test parabolic orbit (e = 1)

        let e = 1.0;
        let parabolic_Ms = vec![-1.0, -0.5, 0.0, 0.5, 1.0];
        for &M in &parabolic_Ms {
            match calc_conic_anomaly_from_mean_anomaly(e,M) {
                Ok(B) => {
                    // Verify Barker's equation: M = tan(f/2)/2 + tan³(f/2)/6
                    // where tan(f/2) = B
                    let computed_M = B/2.0 + B.powi(3)/6.0;
                    assert!((computed_M - M).abs() < EPSILON,
                        "Parabolic orbit: for M = {}, got B = {}, which gives M = {}",
                        M, B, computed_M);
                },
                Err(_) => assert!(false, "Parabolic orbit should not fail for M = {}", M),
        }
    }
    
        // Test hyperbolic orbits (e > 1)

        let hyperbolic_es = vec![1.1, 1.5, 2.0];
        let hyperbolic_Ms = vec![-2.0, -1.0, 0.0, 1.0, 2.0];
        for &e in &hyperbolic_es {
            for &M in &hyperbolic_Ms {
                match calc_conic_anomaly_from_mean_anomaly(e, M) {
                    Ok(H) => {
                        // Check Kepler's equation for hyperbolic orbits: M = e*sinh(H) - H
                        let computed_M = e * H.sinh() - H;
                        assert!((computed_M - M).abs() < EPSILON, 
                            "Hyperbolic orbit: for e = {}, M = {}, got H = {}, which gives M = {}", 
                            e, M, H, computed_M);
                    },
                    Err(_) => assert!(false, "Hyperbolic orbit should not fail for e = {}, M = {}", e, M),
                }
            }
        }
    
        // Test invalid eccentricities

        let e = -0.1;
        match calc_conic_anomaly_from_mean_anomaly(e, 0.0) {
            Ok(_) => assert!(false, "Should reject negative eccentricity"),
            Err(err) => assert_eq!(err, OrbitError::NegativeEccentricity(e)),
        }
    
    }

    /// Testing conic anomaly from true anomaly
    
    #[test]
    fn test_calc_conic_anomaly_from_true_anomaly() {
    
        // Test circular orbits (e = 0)
        
        let e = 0.0;
        let test_anomalies = vec![
            0.0, 
            std::f64::consts::PI / 4.0,
            std::f64::consts::PI / 2.0,
            std::f64::consts::PI,
            3.0 * std::f64::consts::PI / 2.0,
            2.0 * std::f64::consts::PI
        ];
        
        for &f in &test_anomalies {
            match calc_conic_anomaly_from_true_anomaly(e, f) {
                Ok(result) => assert!((result - f).abs() < EPSILON, 
                    "Circular orbit: for f = {}, expected {}, got {}", f, f, result),
                Err(_) => assert!(false, "Circular orbit should not fail for e = {}, f = {}", e, f),
            }
        }
    
        // Test elliptical orbits (0 < e < 1)
        
        let elliptical_es = vec![0.1, 0.5, 0.7, 0.9];
        for &e in &elliptical_es {
            for &f in &test_anomalies {
                match calc_conic_anomaly_from_true_anomaly(e, f) {
                    Ok(E) => {
                        // Verify with the relation: f = 2*atan(sqrt((1+e)/(1-e)) * tan(E/2))
                        let computed_f = 2.0 * ((1.0 + e).sqrt() * (E/2.0).sin()).atan2((1.0 - e).sqrt() * (E/2.0).cos());
                        assert!((computed_f - f).abs() < EPSILON, 
                            "Elliptical orbit: for e = {}, f = {}, got E = {}", e, f, E);
                    },
                    Err(_) => assert!(false, "Elliptical orbit should not fail for e = {}, f = {}", e, f),
                }
            }
        }
    
        // Test parabolic orbit (e = 1)
        
        let e = 1.0;
        let parabolic_fs = vec![-1.0, -0.5, 0.0, 0.5, 1.0];
        for &f in &parabolic_fs {
            match calc_conic_anomaly_from_true_anomaly(e, f) {
                Ok(B) => {
                    // For parabolic orbits, B = tan(f/2)
                    assert!((B - (f/2.0).tan()).abs() < EPSILON,
                        "Parabolic orbit: for f = {}, got B = {}, expected tan(f/2) = {}",
                        f, B, (f/2.0).tan());
                },
                Err(_) => assert!(false, "Parabolic orbit should not fail for f = {}", f),
            }
        }
    
        // Test hyperbolic orbits (e > 1)
        
        let hyperbolic_es = vec![1.1, 1.5, 2.0];
        let hyperbolic_fs = vec![-1.0, -0.5, 0.0, 0.5, 1.0];
        for &e in &hyperbolic_es {
            for &f in &hyperbolic_fs {
                match calc_conic_anomaly_from_true_anomaly(e, f) {
                    Ok(H) => {
                        // Verify using the inverse relation: tan(f/2) = sqrt((e+1)/(e-1)) * tanh(H/2)
                        let tan_f_half = ((e + 1.0)/(e - 1.0)).sqrt() * (H/2.0).tanh();
                        assert!((tan_f_half - (f/2.0).tan()).abs() < EPSILON, 
                            "Hyperbolic orbit: for e = {}, f = {}, got H = {}, which gives tan(f/2) = {}", 
                            e, f, H, tan_f_half);
                    },
                    Err(_) => assert!(false, "Hyperbolic orbit should not fail for e = {}, f = {}", e, f),
                }
            }
        }
    }

    #[test]
    fn test_calc_xyz_from_kepM() {
        // const MU: f64 = 1.0; // Use standardized gravitational parameter
        
        // Test circular orbit (e = 0)
        {
            let a = 1.0;
            let e = 0.0;
            let inc = 0.0;
            let arg = 0.0;
            let node = 0.0;
            let M = 0.0;
            
            match calc_xyz_from_kepM(a, e, inc, arg, node, M, MU_BARY) {
                Ok((pos, vel)) => {
                    // For circular orbit at periapsis:
                    // - Position should be (a, 0, 0)
                    // - Velocity should be (0, sqrt(mu/a), 0)
                    assert!((pos[0] - a).abs() < EPSILON);
                    assert!(pos[1].abs() < EPSILON);
                    assert!(pos[2].abs() < EPSILON);
                    
                    assert!(vel[0].abs() < EPSILON);
                    assert!((vel[1] - (MU_BARY/a).sqrt()).abs() < EPSILON);
                    assert!(vel[2].abs() < EPSILON);
                },
                Err(_) => panic!("Circular orbit calculation failed"),
            }
        }

        // Test elliptical orbit (0 < e < 1)
        {
            let a = 2.0;
            let e = 0.5;
            let inc = 0.0;
            let arg = 0.0;
            let node = 0.0;
            let M = 0.0;  // Test at periapsis
            
            match calc_xyz_from_kepM(a, e, inc, arg, node, M, MU_BARY) {
                Ok((pos, vel)) => {
                    // At periapsis:
                    // - Distance should be a(1-e)
                    // - Velocity should be sqrt(mu*(2/(r) - 1/a))
                    let r = a * (1.0 - e);
                    let v = (MU_BARY * (2.0/r - 1.0/a)).sqrt();
                    
                    assert!((pos.norm() - r).abs() < EPSILON);
                    assert!((vel.norm() - v).abs() < EPSILON);
                },
                Err(_) => panic!("Elliptical orbit calculation failed"),
            }
        }

        // Test hyperbolic orbit (e > 1)
        {
            let a = -2.0;  // Negative semi-major axis for hyperbolic
            let e = 2.0;
            let inc = 0.0;
            let arg = 0.0;
            let node = 0.0;
            let M = 0.0;  // Test at periapsis
            
            match calc_xyz_from_kepM(a, e, inc, arg, node, M, MU_BARY) {
                Ok((pos, vel)) => {
                    // At periapsis:
                    // - Distance should be a(1-e)
                    // - Velocity should be sqrt(mu*(-2/(r) - 1/a))
                    let r = a * (1.0 - e);
                    let v = (MU_BARY * (2.0/r - 1.0/a)).sqrt();
                    
                    assert!((pos.norm() - r.abs()).abs() < EPSILON);
                    assert!((vel.norm() - v).abs() < EPSILON);
                },
                Err(_) => panic!("Hyperbolic orbit calculation failed"),
            }
        }

        // Test orbital rotations
        {
            let a = 1.0;
            let e = 0.0;
            let inc = std::f64::consts::PI/2.0;  // 90 degrees inclination
            let arg = 0.0;
            let node = 0.0;
            let M = 0.0;
            
            match calc_xyz_from_kepM(a, e, inc, arg, node, M, MU_BARY) {
                Ok((pos, vel)) => {
                    // For 90 degree inclination, orbit should be in x-z plane
                    assert!(pos[1].abs() < EPSILON);
                    assert!(vel[1].abs() < EPSILON);
                },
                Err(_) => panic!("Inclined orbit calculation failed"),
            }
        }

    }
}