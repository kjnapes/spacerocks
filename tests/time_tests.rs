#[cfg(test)]
mod tests {
    use spacerocks::time::{Time, TimeFormat, TimeScale};
    use spacerocks::errors::TimeError;

    // Time Creation
    #[test]
    fn test_time_creation() {
        let time = Time::new(2451545.0, &TimeScale::UTC.to_string(), &TimeFormat::JD.to_string())
            .unwrap();
        assert_eq!(time.epoch, 2451545.0);
        assert_eq!(time.timescale, TimeScale::UTC);
        assert_eq!(time.format, TimeFormat::JD);
    }

    // Error Handling 
    #[test]
    fn test_invalid_inputs() {
        // Test new() with invalid inputs
        let result1 = Time::new(2451545.0, "foo", "jd");
        assert!(result1.is_err());
        assert_eq!(result1.unwrap_err().to_string(), 
            "Invalid timescale: foo. Needs to be 'utc', 'tdb', 'tt', or 'tai'.");

        let result2 = Time::new(2451545.0, "utc", "foo");
        assert!(result2.is_err());
        assert_eq!(result2.unwrap_err().to_string(), 
            "Invalid time format: foo. Needs to be 'jd' or 'mjd'.");

        // Test infer_time_format with invalid input
        let result3 = Time::infer_time_format(2459000.5, Some("invalid"));
        match result3 {
            Err(TimeError::InvalidTimeScale(scale)) => assert_eq!(scale, "invalid"),
            _ => panic!("Expected InvalidTimeScale error"),
        }
    }

    // Format Conversions
    #[test]
    fn test_format_conversions() {
        let time_jd = Time::new(2451545.0, "utc", "jd").unwrap();
        assert_eq!(time_jd.mjd(), 51544.5);

        let time_mjd = Time::new(51545.0, "utc", "mjd").unwrap();
        assert_eq!(time_mjd.jd(), 2451545.5);
    }

    // Timescale Conversions (including method chaining)
    #[test]
    fn test_timescale_conversions() {
        let mut time = Time::new(2451545.5, "utc", "jd").unwrap();
        time.tdb();
        assert_eq!(time.timescale, TimeScale::TDB);
        time.tt();
        assert_eq!(time.timescale, TimeScale::TT);
        time.tai();
        assert_eq!(time.timescale, TimeScale::TAI);
        
        let epoch = time.utc().jd();
        assert_eq!(time.timescale, TimeScale::UTC);
        assert_eq!(epoch, 2451545.5);

        let epoch = time.tai().jd();
        assert_eq!(time.timescale, TimeScale::TAI);
    }

    // Time Format Inference 
    #[test]
    fn test_time_format_inference() {
        // Basic JD/MJD inference
        let jd_time = Time::infer_time_format(2459000.5, None).unwrap();
        assert_eq!(jd_time.format, TimeFormat::JD);
        assert_eq!(jd_time.timescale, TimeScale::UTC);  // default timescale

        let mjd_time = Time::infer_time_format(59000.5, None).unwrap();
        assert_eq!(mjd_time.format, TimeFormat::MJD);

        // Explicit timescale
        let tdb_time = Time::infer_time_format(2459000.5, Some("tdb")).unwrap();
        assert_eq!(tdb_time.timescale, TimeScale::TDB);

        // Case insensitivity
        let time1 = Time::infer_time_format(2459000.5, Some("TDB")).unwrap();
        let time2 = Time::infer_time_format(2459000.5, Some("tdb")).unwrap();
        assert_eq!(time1.timescale, time2.timescale);

        // Boundary cases
        assert_eq!(Time::infer_time_format(100_000.0, None).unwrap().format, TimeFormat::MJD);
        assert_eq!(Time::infer_time_format(100_000.1, None).unwrap().format, TimeFormat::JD);
        assert_eq!(Time::infer_time_format(0.0, None).unwrap().format, TimeFormat::MJD);
    }

    // Time object from string
    #[test]
    fn test_from_fuzzy_str() {
        let time1 = Time::from_fuzzy_str("2451545.0 utc jd").unwrap();
        assert_eq!(time1.epoch, 2451545.0);
        assert_eq!(time1.timescale, TimeScale::UTC);
        assert_eq!(time1.format, TimeFormat::JD);

        let time2 = Time::from_fuzzy_str("51544.5 utc mjd").unwrap();
        assert_eq!(time2.epoch, 51544.5);
        assert_eq!(time2.timescale, TimeScale::UTC);
        assert_eq!(time2.format, TimeFormat::MJD);

        let time3 = Time::from_fuzzy_str("2451545.0 tdb jd").unwrap();
        assert_eq!(time3.epoch, 2451545.0);
        assert_eq!(time3.timescale, TimeScale::TDB);
        assert_eq!(time3.format, TimeFormat::JD);

        let time4 = Time::from_fuzzy_str("51544.5 tdb mjd").unwrap();
        assert_eq!(time4.epoch, 51544.5);
        assert_eq!(time4.timescale, TimeScale::TDB);
        assert_eq!(time4.format, TimeFormat::MJD);
    }
}