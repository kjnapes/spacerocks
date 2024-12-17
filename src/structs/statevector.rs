use nalgebra::Vector3;

pub struct StateVector {
    pub position: Vector3<f64>,
    pub velocity: Vector3<f64>,
}

impl StateVector {
    pub fn new(position: Vector3<f64>, velocity: Vector3<f64>) -> StateVector {
        StateVector { position, velocity }
    }

    pub fn x(&self) -> f64 {
        self.position.x
    }

    pub fn y(&self) -> f64 {
        self.position.y
    }

    pub fn z(&self) -> f64 {
        self.position.z
    }

    pub fn vx(&self) -> f64 {
        self.velocity.x
    }

    pub fn vy(&self) -> f64 {
        self.velocity.y
    }

    pub fn vz(&self) -> f64 {
        self.velocity.z
    }
}
