use crate::orbit_type::OrbitType;

pub struct KeplerOrbit {
    pub q: f64,
    pub mee_f: f64,
    pub mee_g: f64,
    pub mee_h: f64,
    pub mee_k: f64,
    pub mean_longitude: f64,
}


impl KeplerOrbit {

    pub fn a(&self) -> Option<f64> {
        match self.orbit_type {
            OrbitType::Parabolic => None,
            _ => Some(self.q / (1.0 - self.e)),
        }
    }

    pub fn p(&self) -> f64 {
        self.q * (1.0 - self.e)
    }

}
