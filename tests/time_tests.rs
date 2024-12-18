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
        // Test in-place modifications
        let mut time = Time::new(2451545.5, "utc", "jd").unwrap();
        time.to_tdb();
        assert_eq!(time.timescale, TimeScale::TDB);
        time.to_tt();
        assert_eq!(time.timescale, TimeScale::TT);
        time.to_tai();
        assert_eq!(time.timescale, TimeScale::TAI);
        time.to_utc();
        assert_eq!(time.timescale, TimeScale::UTC);

        // Test creating new objects
        let time = Time::new(2451545.5, "utc", "jd").unwrap();
        let tdb_time = time.tdb();
        assert_eq!(tdb_time.timescale, TimeScale::TDB);
        assert_eq!(time.timescale, TimeScale::UTC);  // Original unchanged

        let tt_time = tdb_time.tt();
        assert_eq!(tt_time.timescale, TimeScale::TT);
        assert_eq!(tdb_time.timescale, TimeScale::TDB);  // Original unchanged

        let tai_time = tt_time.tai();
        assert_eq!(tai_time.timescale, TimeScale::TAI);
        assert_eq!(tt_time.timescale, TimeScale::TT);  // Original unchanged

        let utc_time = tai_time.utc();
        assert_eq!(utc_time.timescale, TimeScale::UTC);
        assert_eq!(tai_time.timescale, TimeScale::TAI);  // Original unchanged

        // Test change_timescale method
        let mut time = Time::new(2451545.5, "utc", "jd").unwrap();
        let tdb_time = time.change_timescale(TimeScale::TDB);
        assert_eq!(tdb_time.timescale, TimeScale::TDB);
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

    #[test]
    fn test_suggestion_system() {
        // Test timescale suggestions
        let result = Time::new(2451545.0, "utk", "jd");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Did you mean 'utc'?"));

        let result = Time::new(2451545.0, "tbd", "jd");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Did you mean 'tdb'?"));

        // Test format suggestions
        let result = Time::new(2451545.0, "utc", "jdd");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Did you mean 'jd'?"));

        let result = Time::new(2451545.0, "utc", "mjdd");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Did you mean 'mjd'?"));

        // Test case insensitivity
        let result = Time::new(2451545.0, "uTK", "JD");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Did you mean 'utc'?"));

        // Test infer_time_format suggestions
        let result = Time::infer_time_format(2459000.5, Some("utk"));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Did you mean 'utc'?"));

        // Test from_fuzzy_str suggestions
        let result = Time::from_fuzzy_str("2451545.0 utk jd");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Did you mean 'utc'?"));
    }

}