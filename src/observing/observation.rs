use crate::{Time, Observer};

#[derive(Debug, Clone)]
pub enum ObservationType {
    Astrometric { ra: f64, dec: f64 },
    Streak { ra: f64, dec: f64, ra_rate: f64, dec_rate: f64 },
    Radar { ra: f64, dec: f64, range: f64, range_rate: f64 },
    Complete { ra: f64, dec: f64, ra_rate: f64, dec_rate: f64, range: f64, range_rate: f64 },
}

#[derive(Debug, Clone)]
pub struct Observation {
    pub epoch: Time,
    pub observation_type: ObservationType,
    pub observer: Observer,
}

impl Observation {
    pub fn new(epoch: Time, observation_type: ObservationType, observer: Observer) -> Observation {
        Observation { epoch, observation_type, observer }
    }

    pub fn from_astrometric(epoch: Time, ra: f64, dec: f64, observer: Observer) -> Observation {
        Observation::new(epoch, ObservationType::Astrometric { ra, dec }, observer)
    }

    pub fn from_streak(epoch: Time, ra: f64, dec: f64, ra_rate: f64, dec_rate: f64, observer: Observer) -> Observation {
        Observation::new(epoch, ObservationType::Streak { ra, dec, ra_rate, dec_rate }, observer)
    }

    pub fn from_complete(epoch: Time, ra: f64, dec: f64, ra_rate: f64, dec_rate: f64, range: f64, range_rate: f64, observer: Observer) -> Observation {
        Observation::new(epoch, ObservationType::Complete { ra, dec, ra_rate, dec_rate, range, range_rate }, observer)
    }

    pub fn ra(&self) -> f64 {
        match self.observation_type {
            ObservationType::Astrometric { ra, .. } => ra,
            ObservationType::Streak { ra, .. } => ra,
            ObservationType::Radar { ra, .. } => ra,
            ObservationType::Complete { ra, .. } => ra,
        }
    }

    pub fn dec(&self) -> f64 {
        match self.observation_type {
            ObservationType::Astrometric { dec, .. } => dec,
            ObservationType::Streak { dec, .. } => dec,
            ObservationType::Radar { dec, .. } => dec,
            ObservationType::Complete { dec, .. } => dec,
        }
    }

    pub fn ra_rate(&self) -> Option<f64> {
        match self.observation_type {
            ObservationType::Astrometric { .. } => None,
            ObservationType::Streak { ra_rate, .. } => Some(ra_rate),
            ObservationType::Radar { .. } => None,
            ObservationType::Complete { ra_rate, .. } => Some(ra_rate),
        }
    }

    pub fn dec_rate(&self) -> Option<f64> {
        match self.observation_type {
            ObservationType::Astrometric { .. } => None,
            ObservationType::Streak { dec_rate, .. } => Some(dec_rate),
            ObservationType::Radar { .. } => None,
            ObservationType::Complete { dec_rate, .. } => Some(dec_rate),
        }
    }

    pub fn range(&self) -> Option<f64> {
        match self.observation_type {
            ObservationType::Astrometric { .. } => None,
            ObservationType::Streak { .. } => None,
            ObservationType::Radar { range, .. } => Some(range),
            ObservationType::Complete { range, .. } => Some(range),
        }
    }

    pub fn range_rate(&self) -> Option<f64> {
        match self.observation_type {
            ObservationType::Astrometric { .. } => None,
            ObservationType::Streak { .. } => None,
            ObservationType::Radar { range_rate, .. } => Some(range_rate),
            ObservationType::Complete { range_rate, .. } => Some(range_rate),
        }
    }

    pub fn proper_motion(&self) -> Option<f64> {
        let ra_rate = self.ra_rate()?;
        let dec_rate = self.dec_rate()?;
        Some((ra_rate.powi(2) * (self.dec().cos()).powi(2) + dec_rate.powi(2)).sqrt())
    }
}