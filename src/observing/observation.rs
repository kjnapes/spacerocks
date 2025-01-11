use crate::{Time, Observer};

use nalgebra::Vector3;

#[derive(Debug, Clone, PartialEq)]
pub enum ObservationType {
    Astrometry { ra: f64, dec: f64, mag: Option<f64> },
    Streak { ra: f64, dec: f64, ra_rate: f64, dec_rate: f64, mag: Option<f64> },
    Radar { ra: f64, dec: f64, range: f64, range_rate: f64 },
    Complete { ra: f64, dec: f64, ra_rate: f64, dec_rate: f64, range: f64, range_rate: f64, mag: Option<f64> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Observation {
    pub epoch: Time,
    pub observation_type: ObservationType,
    pub observer: Observer,
    // pub filter: Option<String>,
    // pub obsid: Option<String>,
}

impl Observation {
    pub fn new(epoch: Time, observation_type: ObservationType, observer: Observer) -> Observation {
        Observation { epoch, observation_type, observer }
    }

    pub fn from_astrometry(epoch: Time, ra: f64, dec: f64, mag: Option<f64>, observer: Observer) -> Observation {
        Observation::new(epoch, ObservationType::Astrometry { ra, dec, mag }, observer)
    }

    pub fn from_streak(epoch: Time, ra: f64, dec: f64, ra_rate: f64, dec_rate: f64, mag: Option<f64>, observer: Observer) -> Observation {
        Observation::new(epoch, ObservationType::Streak { ra, dec, ra_rate, dec_rate, mag }, observer)
    }

    pub fn from_complete(epoch: Time, ra: f64, dec: f64, ra_rate: f64, dec_rate: f64, range: f64, range_rate: f64, mag: Option<f64>, observer: Observer) -> Observation {
        Observation::new(epoch, ObservationType::Complete { ra, dec, ra_rate, dec_rate, range, range_rate, mag }, observer)
    }

    pub fn ra(&self) -> f64 {
        match self.observation_type {
            ObservationType::Astrometry { ra, .. } => ra,
            ObservationType::Streak { ra, .. } => ra,
            ObservationType::Radar { ra, .. } => ra,
            ObservationType::Complete { ra, .. } => ra,
        }
    }

    pub fn dec(&self) -> f64 {
        match self.observation_type {
            ObservationType::Astrometry { dec, .. } => dec,
            ObservationType::Streak { dec, .. } => dec,
            ObservationType::Radar { dec, .. } => dec,
            ObservationType::Complete { dec, .. } => dec,
        }
    }

    pub fn ra_rate(&self) -> Option<f64> {
        match self.observation_type {
            ObservationType::Astrometry { .. } => None,
            ObservationType::Streak { ra_rate, .. } => Some(ra_rate),
            ObservationType::Radar { .. } => None,
            ObservationType::Complete { ra_rate, .. } => Some(ra_rate),
        }
    }

    pub fn dec_rate(&self) -> Option<f64> {
        match self.observation_type {
            ObservationType::Astrometry { .. } => None,
            ObservationType::Streak { dec_rate, .. } => Some(dec_rate),
            ObservationType::Radar { .. } => None,
            ObservationType::Complete { dec_rate, .. } => Some(dec_rate),
        }
    }

    pub fn range(&self) -> Option<f64> {
        match self.observation_type {
            ObservationType::Astrometry { .. } => None,
            ObservationType::Streak { .. } => None,
            ObservationType::Radar { range, .. } => Some(range),
            ObservationType::Complete { range, .. } => Some(range),
        }
    }

    pub fn range_rate(&self) -> Option<f64> {
        match self.observation_type {
            ObservationType::Astrometry { .. } => None,
            ObservationType::Streak { .. } => None,
            ObservationType::Radar { range_rate, .. } => Some(range_rate),
            ObservationType::Complete { range_rate, .. } => Some(range_rate),
        }
    }

    pub fn mag(&self) -> Option<f64> {
        match self.observation_type {
            ObservationType::Astrometry { mag, .. } => mag,
            ObservationType::Streak { mag, .. } => mag,
            ObservationType::Radar { .. } => None,
            ObservationType::Complete { mag, .. } => mag,
        }
    }

    pub fn proper_motion(&self) -> Option<f64> {
        let ra_rate = self.ra_rate()?;
        let dec_rate = self.dec_rate()?;
        Some((ra_rate.powi(2) * (self.dec().cos()).powi(2) + dec_rate.powi(2)).sqrt())
    }

    pub fn pointing(&self) -> Vector3<f64> {
        let ra = self.ra();
        let dec = self.dec();
        Vector3::new(dec.cos() * ra.cos(), dec.cos() * ra.sin(), dec.sin())
    }
}

// implement a display trait for Observation
impl std::fmt::Display for Observation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.observation_type {
            ObservationType::Astrometry { ra, dec, .. } => write!(f, "Astrometric observation at epoch {} with RA: {} and Dec: {}", self.epoch, ra, dec),
            ObservationType::Streak { ra, dec, ra_rate, dec_rate, .. } => write!(f, "Streak observation at epoch {} with RA: {}, Dec: {}, RA rate: {}, Dec rate: {}", self.epoch, ra, dec, ra_rate, dec_rate),
            ObservationType::Radar { ra, dec, range, range_rate } => write!(f, "Radar observation at epoch {} with RA: {}, Dec: {}, Range: {}, Range rate: {}", self.epoch, ra, dec, range, range_rate),
            ObservationType::Complete { ra, dec, ra_rate, dec_rate, range, range_rate, .. } => write!(f, "Complete observation at epoch {} with RA: {}, Dec: {}, RA rate: {}, Dec rate: {}, Range: {}, Range rate: {}", self.epoch, ra, dec, ra_rate, dec_rate, range, range_rate),
        }
    }
}