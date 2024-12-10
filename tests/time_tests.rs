#[cfg(test)]
mod tests {
    use spacerocks::time::{Time, TimeFormat, TimeScale};
    
    #[test]
    fn test_time_creation() {
        let time = Time::new(2451545.0, &TimeScale::UTC.to_string(), &TimeFormat::JD.to_string())
            .unwrap();

        assert_eq!(time.epoch, 2451545.0);
        assert_eq!(time.timescale, TimeScale::UTC);
        assert_eq!(time.format, TimeFormat::JD);
    }

    #[test]
    fn test_conversion_jd_to_mjd() {
        let time = Time::new(2451545.0, &TimeScale::UTC.to_string(), &TimeFormat::JD.to_string())
            .unwrap();
        let mjd = time.mjd(); // Returns the calculated MJD

        assert_eq!(mjd, 51544.5); // 2451545.0 - 2400000.5
        assert_eq!(time.epoch, 2451545.0); // `epoch` remains unchanged
        assert_eq!(time.timescale, TimeScale::UTC);
        assert_eq!(time.format, TimeFormat::JD); // `format` remains unchanged
    }

    #[test]
    fn test_conversion_mjd_to_jd() {
        let time = Time::new(51545.0, &TimeScale::UTC.to_string(), &TimeFormat::MJD.to_string())
            .unwrap();
        let jd = time.jd(); // Returns the calculated JD

        assert_eq!(jd, 2451545.5); // 51545.0 + 2400000.5
        assert_eq!(time.epoch, 51545.0); // `epoch` remains unchanged
        assert_eq!(time.timescale, TimeScale::UTC);
        assert_eq!(time.format, TimeFormat::MJD); // `format` remains unchanged
    }

    #[test]
    fn test_conversion_to_utc() {
        let mut time = Time::new(2451545.0, &TimeScale::TDB.to_string(), &TimeFormat::JD.to_string())
            .unwrap();
        time.utc(); // Convert TDB to UTC

        
        assert_eq!(time.timescale, TimeScale::UTC);
    }

    #[test]
    fn test_conversion_to_tdb() {
        let mut time = Time::new(2451545.0, &TimeScale::UTC.to_string(), &TimeFormat::JD.to_string())
            .unwrap();
        time.tdb(); // Convert UTC to TDB

        assert_eq!(time.timescale, TimeScale::TDB);
    }

    #[test]
    fn test_method_chaining() {
        let mut time = Time::new(2451545.5, "utc", "jd").unwrap(); // Assuming a constructor or initial value
        let epoch = time.utc().jd(); // Chain my methods

        assert_eq!(epoch, 2451545.5);
    }

    #[test]
    fn test_invalid_timescale() {
        let result = Time::new(2451545.0, "foo", "jd");
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.to_string(), "Invalid timescale: foo. Needs to be 'utc' or 'tdb'.");
        }
    }
    
    #[test]
    fn test_invalid_timeformat() {
        let result = Time::new(2451545.0, "utc", "foo");
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.to_string(), "Invalid time format: foo. Needs to be 'jd' or 'mjd'.");
        }
    }


}
