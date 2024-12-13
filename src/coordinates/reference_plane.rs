use nalgebra::Matrix3;
use crate::constants::{ROTATION_J2000, ROTATION_ECLIPJ2000, ROTATION_INVARIABLE, ROTATION_GALACTIC, ROTATION_FK4};

use serde::{Serialize, Deserialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub enum ReferencePlane {
    J2000,
    #[default]
    ECLIPJ2000,
    INVARIABLE,
    GALACTIC,
    FK4,
}

impl ReferencePlane {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_uppercase().as_str() {
            "J2000" => Ok(ReferencePlane::J2000),
            "ECLIPJ2000" => Ok(ReferencePlane::ECLIPJ2000),
            "INVARIABLE" => Ok(ReferencePlane::INVARIABLE),
            "GALACTIC" => Ok(ReferencePlane::GALACTIC),
            "FK4" => Ok(ReferencePlane::FK4),
            _ => Err(format!("Invalid frame: {}", s))
        }
    }

    // get the rotation matrix for the frame
    pub fn get_rotation_matrix(&self) -> Matrix3<f64> {
        match self {
            ReferencePlane::J2000 => ROTATION_J2000,
            ReferencePlane::ECLIPJ2000 => ROTATION_ECLIPJ2000,
            ReferencePlane::INVARIABLE => ROTATION_INVARIABLE,
            ReferencePlane::GALACTIC => ROTATION_GALACTIC,
            ReferencePlane::FK4 => ROTATION_FK4,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ReferencePlane::J2000 => "J2000",
            ReferencePlane::ECLIPJ2000 => "ECLIPJ2000",
            ReferencePlane::INVARIABLE => "INVARIABLE",
            ReferencePlane::GALACTIC => "GALACTIC",
            ReferencePlane::FK4 => "FK4",
        }
    }

}



impl std::fmt::Display for ReferencePlane {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ReferencePlane::J2000 => write!(f, "J2000"),
            ReferencePlane::ECLIPJ2000 => write!(f, "ECLIPJ2000"),
            ReferencePlane::INVARIABLE => write!(f, "INVARIABLE"),
            ReferencePlane::GALACTIC => write!(f, "GALACTIC"),
            ReferencePlane::FK4 => write!(f, "FK4"),
        }
    }
}