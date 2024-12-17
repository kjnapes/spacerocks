use std::f64::consts::PI;

pub struct KeplerOrbit {
    pub e: f64,
    pub q: f64,
    pub inc: f64,
    pub node: f64,
    pub arg: f64,
    pub true_anomaly: f64,
}

impl KeplerOrbit {

    pub fn new(e: f64, q: f64, inc: f64, node: f64, arg: f64, true_anomaly: f64) -> KeplerOrbit {
        KeplerOrbit { e, q, inc, node, arg, true_anomaly }
    }

    pub fn a(&self) -> f64 {
        self.q / (1.0 - self.e)
    }

    pub fn varpi(&self) -> f64 {
        (self.node + self.arg) % (2.0 * PI)
    }
    
}