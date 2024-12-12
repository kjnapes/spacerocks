use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Properties {
    pub mass: Option<f64>,
    pub absolute_magnitude: Option<f64>,
    pub gslope: Option<f64>,
    pub radius: Option<f64>,
    pub albedo: Option<f64>,
}