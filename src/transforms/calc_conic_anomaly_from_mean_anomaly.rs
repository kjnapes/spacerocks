use crate::errors::OrbitError;
use crate::orbit_type::OrbitType;

/// Calculate the mean anomaly from the eccentric anomaly.
///
/// # Arguments
///
/// * `e` - Eccentricity of the orbit.
/// * `mean_anomaly` - Mean anomaly of the orbit in radians.
///
/// # Returns
///
/// * `Result<f64, OrbitError>` - The conic anomaly in radians.
///
/// # Example
///
/// ```
/// use spacerocks::transforms;
/// let e = 0.5;
/// let mean_anomaly = 0.5;
/// let result = transforms::calc_conic_anomaly_from_mean_anomaly(e, mean_anomaly);
/// ```
#[allow(non_snake_case)]
pub fn calc_conic_anomaly_from_mean_anomaly(e: f64, mean_anomaly: f64) -> Result<f64, OrbitError> {
    let orbit_type = OrbitType::from_eccentricity(e, 1e-10)?; // returns OrbitError error if e < 0.0
    match orbit_type {
        OrbitType::Circular => Ok(kepler_circular(e, mean_anomaly)),
        // OrbitType::Elliptical => Ok(kepler_elliptical(e, mean_anomaly)),
        OrbitType::Elliptical => kepler_elliptical(e, mean_anomaly),
        OrbitType::Parabolic => Ok(kepler_parabolic(e, mean_anomaly)),
        OrbitType::Hyperbolic => Ok(kepler_hyperbolic(e, mean_anomaly)),
        OrbitType::Radial => unreachable!(),
    }

}

fn kepler_circular(_e: f64, mean_anomaly: f64) -> f64 {
    mean_anomaly
}

// fn kepler_elliptical(e: f64, mean_anomaly: f64) -> f64 {

//     let mut flag = false;
//     let mut mean_anomaly = mean_anomaly;
//     if mean_anomaly > std::f64::consts::PI {
//         mean_anomaly = 2.0 * std::f64::consts::PI - mean_anomaly;
//         flag = true;
//     }

//     // Define initial estimate
//     let sin_mean_anomaly = mean_anomaly.sin();
//     let mut eccentric_anomaly = e * sin_mean_anomaly + f64::max(mean_anomaly, e * (sin_mean_anomaly + 0.591));
    

//     // Perform Newton-Raphson estimate
//     for _ in 0..10 {

//         // Compute f(E), f'(E), f''(E) and f'''(E), avoiding recomputation of sine and cosine.
//         let esin_eccentric_anomaly = e * eccentric_anomaly.sin();
//         let ecos_eccentric_anomaly = e * eccentric_anomaly.cos();
        
//         let f = eccentric_anomaly - esin_eccentric_anomaly - mean_anomaly;

//         if f.abs() < 1.0e-15 {
//             if flag {
//                 eccentric_anomaly = 2.0 * std::f64::consts::PI - eccentric_anomaly;
//             }
//             return eccentric_anomaly;
//         }

//         let first = 1.0 - ecos_eccentric_anomaly;
//         let second = esin_eccentric_anomaly;
//         let third = ecos_eccentric_anomaly;

//         let delta_i1 = -f / first;
//         let delta_i2 = -f / (first + 0.5 * delta_i1 * second);
//         let delta_i3 = -f / (first + 0.5 * delta_i2 * second + 1.0/6.0 * third * delta_i2 * delta_i2);
        
//         // Update E
//         eccentric_anomaly += delta_i3;
//     }
//     eccentric_anomaly
// }

    fn kepler_elliptical(e: f64, mean_anomaly: f64) -> Result<f64, OrbitError> {
        let mut flag = false;
        let mut mean_anomaly = mean_anomaly;
        if mean_anomaly > std::f64::consts::PI {
            mean_anomaly = 2.0 * std::f64::consts::PI - mean_anomaly;
            flag = true;
        }

        // Define initial estimate
        let sin_mean_anomaly = mean_anomaly.sin();
        let mut eccentric_anomaly = e * sin_mean_anomaly + f64::max(mean_anomaly, e * (sin_mean_anomaly + 0.591));
        
        // Perform Newton-Raphson estimate
        for _ in 0..10 {
            let esin_eccentric_anomaly = e * eccentric_anomaly.sin();
            let ecos_eccentric_anomaly = e * eccentric_anomaly.cos();
            
            let f = eccentric_anomaly - esin_eccentric_anomaly - mean_anomaly;

            if f.abs() < 1.0e-15 {
                if flag {
                    eccentric_anomaly = 2.0 * std::f64::consts::PI - eccentric_anomaly;
                }
                return Ok(eccentric_anomaly);
            }

            let first = 1.0 - ecos_eccentric_anomaly;
            let second = esin_eccentric_anomaly;
            let third = ecos_eccentric_anomaly;

            let delta_i1 = -f / first;
            let delta_i2 = -f / (first + 0.5 * delta_i1 * second);
            let delta_i3 = -f / (first + 0.5 * delta_i2 * second + 1.0/6.0 * third * delta_i2 * delta_i2);
            
            eccentric_anomaly += delta_i3;
        }

        // If we get here, we didn't converge
        Err(OrbitError::ConvergenceFailure(e, mean_anomaly))
    }

    /// Calculate the parabolic eccentric anomaly from the mean anomaly.
    fn kepler_parabolic(_e: f64, mean_anomaly: f64) -> f64 {
        let x = (3.0 * mean_anomaly + (4.0 + 9.0 * mean_anomaly * mean_anomaly).sqrt()).cbrt();
        let y = (2.0_f64).cbrt();
        x/y - y/x 
    }

    /// Calculate the hyperbolic eccentric anomaly from the mean anomaly.
    fn kepler_hyperbolic(e: f64, mean_anomaly: f64) -> f64 {

        let mut eccentric_anomaly = mean_anomaly / mean_anomaly.abs() * (2.0 * mean_anomaly.abs() / e + 1.8).ln();
        let mut f = eccentric_anomaly - e * eccentric_anomaly.sinh() + mean_anomaly;
        for _ in 1..100 {
            eccentric_anomaly = eccentric_anomaly - f / (1.0 - e * eccentric_anomaly.cosh());
            f = eccentric_anomaly - e * eccentric_anomaly.sinh() + mean_anomaly;
            if f.abs() < 1.0e-15 {
                break;
            }
        }
        eccentric_anomaly
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_eccentric_anomaly_from_mean_anomaly() {
        let e = 0.0;
        let mean_anomaly = 0.0;
        match calc_eccentric_anomaly_from_mean_anomaly(e, mean_anomaly) {
            Ok(result) => assert_eq!(result, 0.0),
            Err(_) => assert!(false),
        }

        let e = 0.5;
        let mean_anomaly = 0.5;
        match calc_eccentric_anomaly_from_mean_anomaly(e, mean_anomaly) {
            Ok(result) => assert_eq!(result, 0.5),
            Err(_) => assert!(false),
        }

        let e = -0.1;
        let mean_anomaly = 0.0;
        match calc_eccentric_anomaly_from_mean_anomaly(e, mean_anomaly) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }
}