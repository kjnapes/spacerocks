use crate::SpaceRock;
use crate::ReferencePlane;
use crate::Origin;

use crate::observing::observer::Observer;
use crate::constants::{DEG_TO_RAD, M_TO_AU, EQUAT_RAD};
use crate::OBSERVATORIES;
use crate::time::Time;

use nalgebra::Vector3;


#[derive(Clone, Debug)]
pub enum Observatory {
    GroundObservatory { obscode: String, lon: f64, lat: f64, rho: f64 },
    SpaceTelecope { name: String },
}

impl Observatory {

    pub fn from_obscode(obscode: &str) -> Result<Self, &'static str> {
        let obscode = obscode.to_uppercase();
        match OBSERVATORIES.get(&obscode) {
            Some(obs) => {
                let lon = obs.0;
                let lat = obs.1.atan2(obs.2);
                let rho = (obs.1 * obs.1 + obs.2 * obs.2).sqrt();
                return Ok(Observatory::GroundObservatory { obscode: obscode, lon, lat, rho })
            },
            None => {
                return Err("Observatory not found")
            }
        }
    }

    // pub fn from_parallax(lon: f64, rho_cos_lat: f64, rho_sin_lat: f64) -> Self {
    //     let o = Observatory::GroundObservatory { obscode: "PARALLAX".to_string(), lon, lat: 0.0, rho: 0.0 };
    // }

    // pub fn at(&self, epoch: &Time, origin: &Origin) -> Observer {
    //     match self.ObservatoryType {
    //         ObservatoryType::Topocentric { lon, rho_cos_lat, rho_sin_lat } => {
    //             return self.at_topocentric(epoch, origin, lon, rho_cos_lat, rho_sin_lat)
    //         },
    //         ObservatoryType::SpaceTelecope { name } => {
    //             return self.at_space_telescope(epoch, origin, name)
    //         }
    //     }
    //     let mut earth = SpaceRock::from_spice("earth", epoch, &CoordinateFrame::J2000, origin);
    //     let [d_pos, d_vel] = compute_topocentric_correction(self.lon, self.rho_sin_lat, self.rho_cos_lat, epoch.epoch);
    //     earth.position += d_pos;
    //     earth.velocity += d_vel;
    //     Observer::from_ground(earth.position, earth.velocity, earth.epoch, &earth.frame, &earth.origin, self.lat(), self.lon, self.rho())
    // }

    pub fn at(&self, epoch: &Time) -> Result<Observer, Box<dyn std::error::Error>> {
        match self {
            Observatory::GroundObservatory { obscode, lon, lat, rho } => {
                let mut earth = SpaceRock::from_spice("earth", epoch, "J2000", "ssb")?;
                let [d_pos, d_vel] = compute_topocentric_correction(*lon, *lat, *rho, epoch.epoch);
                earth.position += d_pos;
                earth.velocity += d_vel;
                Ok(Observer { spacerock: earth, observatory: self.clone() })
            },
            Observatory::SpaceTelecope { name } => {
                let rock = SpaceRock::from_spice(&name, epoch, "J2000", "ssb")?;
                Ok(Observer { spacerock: rock, observatory: self.clone() })
            }
        }
    }

    

    // pub fn at(&self, epoch: &Time) -> Result<Observer, Box<dyn std::error::Error>> {
    //     match self {
    //         Observatory::GroundObservatory {obscode, lon, lat, rho} => {
    //             let mut earth = SpaceRock::from_spice("earth", epoch, "J2000", "ssb")?;
    //             let [d_pos, d_vel] = compute_topocentric_correction(self.lon, self.rho_sin_lat, self.rho_cos_lat, epoch.epoch);
    //             earth.position += d_pos;
    //             earth.velocity += d_vel;
    //             Ok(Observer { spacerock: earth, observatory: self })
    //         },
    //         Observatory::SpaceTelecope {name} => {
    //             let rock = SpaceRock::from_spice(&self.name, epoch, "J2000", "ssb")?;
    //             Ok(Observer { spacerock: rock, observatory: self })
    //         }
    //     }
    // }

    pub fn name(&self) -> String {
        match self {
            Observatory::GroundObservatory { obscode, .. } => obscode.clone(),
            Observatory::SpaceTelecope { name } => name.clone(),
        }
    }

    pub fn lon(&self) -> Option<f64> {
        match self {
            Observatory::GroundObservatory { lon, .. } => Some(*lon),
            Observatory::SpaceTelecope { .. } => None,
        }
    }

    pub fn lat(&self) -> Option<f64> {
        match self {
            Observatory::GroundObservatory { lat, .. } => Some(*lat),
            Observatory::SpaceTelecope { .. } => None,
        }
    }

    pub fn rho(&self) -> Option<f64> {
        match self {
            Observatory::GroundObservatory { rho, .. } => Some(*rho),
            Observatory::SpaceTelecope { .. } => None,
        }
    }

}

// impl Observatory {

    


//     pub fn lat(&self) -> f64 {
//         return self.rho_sin_lat.atan2(self.rho_cos_lat)
//     }

//     pub fn rho(&self) -> f64 {
//         return (self.rho_sin_lat * self.rho_sin_lat + self.rho_cos_lat * self.rho_cos_lat).sqrt()
//     }

    

//     pub fn local_sidereal_time(&self, epoch: f64) -> f64 {
//         return compute_local_sidereal_time(epoch, self.lon)
//     }

// }

fn compute_local_sidereal_time(epoch: f64, lon: f64) -> f64 {
    let t = (epoch - 2451545.0) / 36525.0;
    let mut theta = 280.46061837 + 360.98564736629 * (epoch - 2451545.0) + (0.000387933 * t * t) - (t * t * t / 38710000.0);
    theta *= DEG_TO_RAD;
    return theta + lon
}

fn sidereal_rate(epoch: f64) -> f64 {
    let T = (epoch - 2451545.0) / 36525.0;
    let Tprime = 1.0 / 36525.0;
    let theta_dot = 360.98564736629 + 2.0 * 0.000387933 * T * Tprime + 3.0 * T * T * Tprime / 38710000.0;
    return theta_dot * DEG_TO_RAD
}

fn compute_topocentric_correction(lon: f64, rho_sin_lat: f64, rho_cos_lat: f64, epoch: f64) -> [Vector3<f64>; 2] {

    let phi = compute_local_sidereal_time(epoch, lon);
    let phi_rate = sidereal_rate(epoch);
    
    let sin_lon = phi.sin();
    let cos_lon = phi.cos();

    let sin_lon_prime = cos_lon * phi_rate;
    let cos_lon_prime = -sin_lon * phi_rate;
        
    let dx = rho_cos_lat * cos_lon * EQUAT_RAD;
    let dy = rho_cos_lat * sin_lon * EQUAT_RAD;
    let dz = rho_sin_lat * EQUAT_RAD;

    let dvx = cos_lon_prime * rho_cos_lat * EQUAT_RAD;
    let dvy = sin_lon_prime * rho_cos_lat * EQUAT_RAD;
    let dvz = 0.0;

    let d_pos = Vector3::new(dx, dy, dz) * M_TO_AU; // AU
    let d_vel = Vector3::new(dvx, dvy, dvz) * M_TO_AU; // AU/day

    return [d_pos, d_vel];

}
