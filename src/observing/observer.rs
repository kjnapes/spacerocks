use crate::observing::observatory::Observatory;
use crate::SpaceRock;

#[derive(Debug, Clone)]
pub struct Observer {
    pub spacerock: SpaceRock,
    pub observatory: Observatory,
}

impl Observer {

    pub fn lat(&self) -> Option<f64> {
        self.observatory.lat()
    }

    pub fn lon(&self) -> Option<f64> {
        self.observatory.lon()
    }
}