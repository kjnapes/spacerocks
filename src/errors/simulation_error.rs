use crate::{Time, Origin};

#[derive(Debug)]
pub enum SimulationError {
    OriginMismatch(Origin, Origin),
    EpochMismatch(Time, Time, String),
}

impl std::fmt::Display for SimulationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimulationError::OriginMismatch(op, osim) => write!(f, "The origin of the particle ({}) did not match the simulation origin ({}), and was not found in perturbers.", op, osim),
            SimulationError::EpochMismatch(tp, tsim, p) => write!(f, "The epoch of particle {} ({:?}) did not match the simulation epoch ({:?}).", p, tp, tsim),
        }
    }
}

impl std::error::Error for SimulationError {}