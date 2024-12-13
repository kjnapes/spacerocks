use std::ops::{AddAssign, Add, Sub};
// use std::collections::HashMap;
use chrono::{Utc, DateTime};
// use chrono::TimeZone;
// use crate::time::leapseconds::LEAP_SECONDS;
use crate::time::timescale::TimeScale;
use crate::time::timeformat::TimeFormat;
use crate::errors::TimeError;
// use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use crate::time::conversions::*;



// finish docs
// bind to python
// benchmarks?
// levenshtein distance -> did you mean this?

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Time {
    pub epoch: f64,
    pub timescale: TimeScale,
    pub format: TimeFormat,
}


impl Time {
    /// Create a new `Time` object.
    ///
    /// # Arguments
    ///
    /// * `epoch` - The epoch of the time (JD or MJD).
    /// * `timescale` - The timescale of the time (UTC or TDB).
    /// * `format` - The format of the time (JD or MJD).
    ///
    /// # Returns
    ///
    /// * `Result<Time, TimeError>` - The time object.
    ///
    /// # Example
    ///
    /// ```
    /// use spacerocks::Time;
    ///
    /// let t = Time::new(2451545.0, "UTC", "JD");
    /// ```
    pub fn new(epoch: f64, timescale: &str, format: &str) -> Result<Self, TimeError> {
        let timescale = match timescale.to_lowercase().as_str() {
            "utc" => TimeScale::UTC,
            "tdb" => TimeScale::TDB,
            "tt" => TimeScale::TT,
            "tai" => TimeScale::TAI,
            _ => return Err(TimeError::InvalidTimeScale(timescale.to_string())),
        };
        let format = match format.to_lowercase().as_str() {
            "jd" => TimeFormat::JD,
            "mjd" => TimeFormat::MJD,
            _ => return Err(TimeError::InvalidTimeFormat(format.to_string())),
        };
        let t = Time {
            epoch,
            timescale,
            format,
        };
        Ok(t)
    }

    
    /// Create a new `Time` object from the current time.
    ///
    /// # Returns
    ///
    /// * `Time` - The time object with the current time in UTC and JD format.
    /// 
    /// # Example
    ///
    /// ```
    /// let t = Time::now();
    /// ```
    pub fn now() -> Self {
        let now = Utc::now();
        let x = now.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        Time {
            epoch: isot_to_julian(&x),
            timescale: TimeScale::UTC,
            format: TimeFormat::JD,
        }
    }

    /// Create a new `Time` object from a fuzzy string.
    ///
    /// # Arguments
    ///
    /// * `s` - A string representing a time in the format "epoch timescale format".
    ///
    /// # Returns
    ///
    /// * `Result<Time, TimeError>` - The time object.
    ///
    /// # Example
    ///
    /// ```
    /// let t = Time::from_fuzzy_str("2451545.0 UTC JD");
    /// ```
    pub fn from_fuzzy_str(s: &str) -> Result<Self, TimeError> {
        let s = s.to_lowercase();
        if s == "now" {
            return Ok(Time::now());
        }
        let mut parts = s.split_whitespace();
        let epoch = parts.next().unwrap().parse::<f64>().unwrap();
        let timescale = parts.next().unwrap();
        let format = parts.next().unwrap();
        Time::new(epoch, timescale, format)
    }

    /// Infer the time format from the epoch and create a new `Time` object.
    ///
    /// # Arguments
    ///
    /// * `epoch` - The epoch of the time.
    /// * `timescale` - The timescale of the time as a str.
    ///
    /// # Returns
    ///
    /// * `Result<Time, TimeError>` - The time object.
    ///
    /// # Example
    ///
    /// ```
    /// let t = Time::infer_time_format(2451545.0, None);
    /// ```
    pub fn infer_time_format(epoch: f64, timescale: Option<&str>) -> Result<Self, TimeError> {
        let timescale = match timescale {
            Some(ts) => match ts.to_lowercase().as_str() {
                "utc" => TimeScale::UTC,
                "tdb" => TimeScale::TDB,
                "tt" => TimeScale::TT,
                "tai" => TimeScale::TAI,
                _ => return Err(TimeError::InvalidTimeScale(ts.to_string())),
            },
            None => TimeScale::UTC,
        };
        
        let format = if epoch > 100_000.0 {
            TimeFormat::JD
        } else {
            TimeFormat::MJD
        };
        
        Ok(Time {
            epoch,
            timescale,
            format,
        })
    }


    // Setters for the timescale of the time object


    /// Convert the time to UTC.
    ///
    /// # Returns
    ///
    /// * `Time` - The time object with the timescale set to UTC.
    pub fn utc(&mut self) -> &mut Self {
        match self.timescale {
            TimeScale::UTC => {}, // Already UTC
            TimeScale::TDB => {
                self.epoch = tdb_to_utc(self.epoch);
                self.timescale = TimeScale::UTC;
            },
            TimeScale::TT => {
                self.epoch = tt_to_utc(self.epoch);
                self.timescale = TimeScale::UTC;
            },
            TimeScale::TAI => {
                self.epoch = tai_to_utc(self.epoch);
                self.timescale = TimeScale::UTC;
            },
        }
        self
    }

    /// Convert the time to TDB.
    ///
    /// # Returns
    ///
    /// * `Time` - The time object with the timescale set to TDB.
    pub fn tdb(&mut self) -> &mut Self {
        match self.timescale {
            TimeScale::UTC => {
                self.epoch = utc_to_tdb(self.epoch);
                self.timescale = TimeScale::TDB;
            },
            TimeScale::TDB => {}, // Already TDB
            TimeScale::TT => {
                self.epoch = tt_to_tdb(self.epoch);
                self.timescale = TimeScale::TDB;
            },
            TimeScale::TAI => {
                // Convert TAI -> TT -> TDB
                let tt = tai_to_tt(self.epoch);
                self.epoch = tt_to_tdb(tt);
                self.timescale = TimeScale::TDB;
            },
        }
        self
    }

    /// Convert the time to TT.
    ///
    /// # Returns
    ///
    /// * `Time` - The time object with the timescale set to TT.
    pub fn tt(&mut self) -> &mut Self {
        match self.timescale {
            TimeScale::UTC => {
                self.epoch = utc_to_tt(self.epoch);
                self.timescale = TimeScale::TT;
            },
            TimeScale::TDB => {
                self.epoch = tdb_to_tt(self.epoch);
                self.timescale = TimeScale::TT;
            },
            TimeScale::TT => {}, // Already TT
            TimeScale::TAI => {
                self.epoch = tai_to_tt(self.epoch);
                self.timescale = TimeScale::TT;
            },
        }
        self
    }

    /// Convert the time to TAI.
    ///
    /// # Returns
    ///
    /// * `Time` - The time object with the timescale set to TAI.
    pub fn tai(&mut self) -> &mut Self {
        match self.timescale {
            TimeScale::UTC => {
                self.epoch = utc_to_tai(self.epoch);
                self.timescale = TimeScale::TAI;
            },
            TimeScale::TDB => {
                // Convert TDB -> TT -> TAI
                let tt = tdb_to_tt(self.epoch);
                self.epoch = tt_to_tai(tt);
                self.timescale = TimeScale::TAI;
            },
            TimeScale::TT => {
                self.epoch = tt_to_tai(self.epoch);
                self.timescale = TimeScale::TAI;
            },
            TimeScale::TAI => {}, // Already TAI
        }
        self
    }

    // Getters for the time format of the time object

    /// Return the time as a JD.
    ///
    /// # Returns
    ///
    /// * `f64` - The time as a JD.
    pub fn jd(&self) -> f64 {
        match self.format {
            TimeFormat::JD => self.epoch,
            TimeFormat::MJD => self.epoch + 2400000.5, // Convert MJD to JD
        }
    }

    /// Return the time as an MJD.
    ///
    /// # Returns
    ///
    /// * `f64` - The time as an MJD.
    pub fn mjd(&self) -> f64 {
        match self.format {
            TimeFormat::JD => self.epoch - 2400000.5, // Convert JD to MJD
            TimeFormat::MJD => self.epoch,
        }
    }


    pub fn change_timescale(&mut self, timescale: TimeScale) -> &mut Self {
        match timescale {
            TimeScale::UTC => self.utc(),
            TimeScale::TDB => self.tdb(),
            TimeScale::TT => self.tt(),
            TimeScale::TAI => self.tai(),
        }
    }

    /// Convert the time to a human-readable calendar date.
    /// 
    /// # Returns
    ///
    /// * `String` - A string representing the date in the format "DD Mon YYYY"
    pub fn calendar(&self) -> String {
        // clone the time object and convert to UTC
        let mut time = self.clone();
        jd_to_calendar(&time.utc().jd())
    }

}

impl Sub<&Time> for &Time {
    type Output = f64;

    fn sub(self, other: &Time) -> f64 {
        if self.timescale != other.timescale {
            panic!("Cannot subtract timescales: {} and {}", self.timescale, other.timescale);
        }
        self.epoch - other.epoch
    }
}

impl Sub<f64> for Time {
    type Output = Time;

    fn sub(self, dt: f64) -> Time {
        Time {
            epoch: self.epoch - dt,
            timescale: self.timescale.clone(),
            format: self.format.clone(),
        }
    }

}

impl Add<f64> for Time {
    type Output = Time;

    fn add(self, dt: f64) -> Time {
        Time {
            epoch: self.epoch + dt,
            timescale: self.timescale.clone(),
            format: self.format.clone(),
        }
    }

}


impl AddAssign<f64> for Time {
    fn add_assign(&mut self, dt: f64) {
        self.epoch += dt;
    }
}


// Need to get this back 

// fn get_isot_now() -> String {
//     let now = Utc::now();
//     let x = now.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
//     return x;
// }




