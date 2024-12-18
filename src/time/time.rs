use std::ops::{AddAssign, Add, Sub};
// use std::collections::HashMap;
use chrono::{Utc, TimeZone, DateTime};
// use chrono::TimeZone;
// use crate::time::leapseconds::LEAP_SECONDS;
use crate::time::timescale::TimeScale;
use crate::time::timeformat::TimeFormat;
use crate::errors::TimeError;
use serde::{Serialize, Deserialize};
use crate::time::conversions::*;
use strsim::jaro_winkler;
use strsim::jaro;
use strsim::levenshtein;
use strsim::damerau_levenshtein;





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

    // pub fn new(epoch: f64, timescale: &str, format: &str) -> Result<Self, TimeError> {
     
    //     let timescale = match timescale.to_uppercase().as_str() {
    //         "UTC" => TimeScale::UTC,
    //         "TDB" => TimeScale::TDB,
    //         "TT" => TimeScale::TT,
    //         "TAI" => TimeScale::TAI,
    //         _ => {
    //             let suggestion = Self::find_closest_match(
    //                 &timescale.to_uppercase(),
    //                 TimeScale::variants()
    //             ).map(|s| format!("Did you mean '{}'?", s.to_lowercase()))
    //              .unwrap_or_default();
    //             return Err(TimeError::InvalidTimeScale(format!(
    //                 "'{}'. {}",
    //                 timescale.to_string(),
    //                 suggestion
    //             )));
    //         }
    //     };
     
    //     let format = match format.to_uppercase().as_str() {
    //         "JD" => TimeFormat::JD,
    //         "MJD" => TimeFormat::MJD,
    //         _ => {
    //             let suggestion = Self::find_closest_match(
    //                 &format.to_uppercase(),
    //                 TimeFormat::variants()
    //             ).map(|s| format!("Did you mean '{}'?", s.to_lowercase()))
    //              .unwrap_or_default();
    //             return Err(TimeError::InvalidTimeFormat(format!(
    //                 "'{}'. {}",
    //                 format.to_string(),
    //                 suggestion
    //             )));
    //         }
    //     };
     
    //     Ok(Time {
    //         epoch,
    //         timescale: timescale,
    //         format: format,
    //     })
    //  }

    
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
    // pub fn infer_time_format(epoch: f64, timescale: Option<&str>) -> Result<Self, TimeError> {
    //     let timescale = match timescale {
    //         Some(ts) => match ts.to_lowercase().as_str() {
    //             "utc" => TimeScale::UTC,
    //             "tdb" => TimeScale::TDB,
    //             "tt" => TimeScale::TT,
    //             "tai" => TimeScale::TAI,
    //             _ => return Err(TimeError::InvalidTimeScale(ts.to_string())),
    //         },
    //         None => TimeScale::UTC,
    //     };
        
    //     let format = if epoch > 100_000.0 {
    //         TimeFormat::JD
    //     } else {
    //         TimeFormat::MJD
    //     };
        
    //     Ok(Time {
    //         epoch,
    //         timescale,
    //         format,
    //     })
    // }

    pub fn infer_time_format(epoch: f64, timescale: Option<&str>) -> Result<Self, TimeError> {
        let timescale = timescale.unwrap_or("UTC");
        let format = if epoch > 100_000.0 { "JD" } else { "MJD" };
        
        Time::new(epoch, timescale, format)
    }


    // Return a new Time object with converted timescale


    /// Create a new time object with UTC timescale
    ///
    /// # Returns
    ///
    /// * `Time` - New time object with the timescale set to UTC.
    ///
    /// ```
    /// let tdb_time = Time::new(2456205.5, "tdb", "jd").unwrap();
    /// let utc_time = tdb_time.utc();  // Creates new Time object in UTC
    /// assert!(tdb_time.epoch != utc_time.epoch);  // Epochs differ due to timescale conversion
    /// ```
    pub fn utc(&self) -> Time {
        let mut new_time = self.clone();
        match self.timescale {
            TimeScale::UTC => new_time, // Already UTC
            TimeScale::TDB => {
                new_time.epoch = tdb_to_utc(self.epoch);
                new_time.timescale = TimeScale::UTC;
                new_time
            },
            TimeScale::TT => {
                new_time.epoch = tt_to_utc(self.epoch);
                new_time.timescale = TimeScale::UTC;
                new_time
            },
            TimeScale::TAI => {
                new_time.epoch = tai_to_utc(self.epoch);
                new_time.timescale = TimeScale::UTC;
                new_time
            },
        }
    }

    /// Create a new time object with TDB timescale
    ///
    /// # Returns
    ///
    /// * `Time` - New time object with the timescale set to TDB.
    pub fn tdb(&self) -> Time {
        let mut new_time = self.clone();
        match self.timescale {
            TimeScale::UTC => {
                new_time.epoch = utc_to_tdb(self.epoch);
                new_time.timescale = TimeScale::TDB;
                new_time
            },
            TimeScale::TDB => new_time, // Already TDB
            TimeScale::TT => {
                new_time.epoch = tt_to_tdb(self.epoch);
                new_time.timescale = TimeScale::TDB;
                new_time
            },
            TimeScale::TAI => {
                // Convert TAI -> TT -> TDB
                let tt = tai_to_tt(self.epoch);
                new_time.epoch = tt_to_tdb(tt);
                new_time.timescale = TimeScale::TDB;
                new_time
            },
        }
    }

    /// Create a new time object with TT timescale
    ///
    /// # Returns
    ///
    /// * `Time` - New time object with the timescale set to TT.
    pub fn tt(&self) -> Time {
        let mut new_time = self.clone();
        match self.timescale {
            TimeScale::UTC => {
                new_time.epoch = utc_to_tt(self.epoch);
                new_time.timescale = TimeScale::TT;
                new_time
            },
            TimeScale::TDB => {
                new_time.epoch = tdb_to_tt(self.epoch);
                new_time.timescale = TimeScale::TT;
                new_time
            },
            TimeScale::TT => new_time, // Already TT
            TimeScale::TAI => {
                new_time.epoch = tai_to_tt(self.epoch);
                new_time.timescale = TimeScale::TT;
                new_time
            },
        }
    }

    /// Create a new time object with TAI timescale
    ///
    /// # Returns
    ///
    /// * `Time` - New time object with the timescale set to TAI.
    pub fn tai(&self) -> Time {
        let mut new_time = self.clone();
        match self.timescale {
            TimeScale::UTC => {
                new_time.epoch = utc_to_tai(self.epoch);
                new_time.timescale = TimeScale::TAI;
                new_time
            },
            TimeScale::TDB => {
                // Convert TDB -> TT -> TAI
                let tt = tdb_to_tt(self.epoch);
                new_time.epoch = tt_to_tai(tt);
                new_time.timescale = TimeScale::TAI;
                new_time
            },
            TimeScale::TT => {
                new_time.epoch = tt_to_tai(self.epoch);
                new_time.timescale = TimeScale::TAI;
                new_time
            },
            TimeScale::TAI => new_time, // Already TAI
        }
    }

    // Modify the timescale of an object in-place

    /// Convert the time object to UTC timescale in place
    ///
    /// Modifies the time object by converting its epoch and timescale to TT.
    ///
    /// # Example
    ///
    /// ```
    /// let mut time = Time::new(2456205.5, "tdb", "jd").unwrap();
    /// time.to_utc();  // Converts time to UTC in place
    /// assert_eq!(time.timescale, TimeScale::UTC);
    /// ```
    pub fn to_utc(&mut self) {
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
    }

    /// Convert the time object to UTC timescale in place
    ///
    /// Modifies the time object by converting its epoch and timescale to TDB.
    pub fn to_tdb(&mut self) {
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
    }

    /// Convert the time object to UTC timescale in place
    ///
    /// Modifies the time object by converting its epoch and timescale to TT.
    pub fn to_tt(&mut self) {
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
    }

    /// Convert the time object to UTC timescale in place
    ///
    /// Modifies the time object by converting its epoch and timescale to TAI.
    pub fn to_tai(&mut self) {
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
            TimeScale::UTC => self.to_utc(),
            TimeScale::TDB => self.to_tdb(),
            TimeScale::TT => self.to_tt(),
            TimeScale::TAI => self.to_tai(),
        }
        self
    }

    /// Convert the time to a human-readable calendar date.
    /// 
    /// # Returns
    ///
    /// * `String` - A string representing the date in the format "DD Mon YYYY"
    pub fn calendar(&self) -> String {
        // clone the time object and convert to UTC
        // let mut time = self.clone();
        jd_to_calendar(&self.utc().jd())
    }

}


/// Implement the `Display` trait for `Time`.
impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.epoch, self.timescale, self.format)
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




