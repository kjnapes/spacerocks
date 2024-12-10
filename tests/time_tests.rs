#[cfg(test)]
mod tests {
    use spacerocks::time::{Time, TimeFormat, TimeScale};
    
    #[test]
    fn test_time_creation() {
        let mut time = Time::new(2451545.0, &TimeScale::UTC.to_string(), &TimeFormat::JD.to_string());

        assert_eq!(time.epoch, 2451545.0);
        assert_eq!(time.timescale, TimeScale::UTC);
        assert_eq!(time.format, TimeFormat::JD);
    }

    #[test]
    fn test_conversion_jd_to_mjd() {
        let mut time = Time::new(2451545.0, &TimeScale::UTC.to_string(), &TimeFormat::JD.to_string());
        time.mjd(); // Convert JD to MJD

        assert_eq!(time.epoch, 51544.5); // 2451545.0 - 2400000.5
        assert_eq!(time.timescale, TimeScale::UTC);
        assert_eq!(time.format, TimeFormat::MJD);
    }

    #[test]
    fn test_conversion_mjd_to_jd() {
        let mut time = Time::new(51545.0, &TimeScale::UTC.to_string(), &TimeFormat::MJD.to_string());
        time.jd(); // Convert MJD to JD

        assert_eq!(time.epoch, 2451545.5); // 51545.0 + 2400000.5
        assert_eq!(time.timescale, TimeScale::UTC);
        assert_eq!(time.format, TimeFormat::JD);
    }

    #[test]
    fn test_conversion_to_utc() {
        let mut time = Time::new(2451545.0, &TimeScale::TDB.to_string(), &TimeFormat::JD.to_string());
        time.utc(); // Convert TDB to UTC

        
        assert_eq!(time.timescale, TimeScale::UTC);
    }

    #[test]
    fn test_conversion_to_tdb() {
        let mut time = Time::new(2451545.0, &TimeScale::UTC.to_string(), &TimeFormat::JD.to_string());
        time.tdb(); // Convert UTC to TDB

        assert_eq!(time.timescale, TimeScale::TDB);
    }

    #[test]
    fn test_method_chaining() {
        let mut time = Time::new(2451545.5, "utc", "jd"); // Assuming a constructor or initial value
        time.mjd().jd().mjd(); // Convert JD to MJD, MJD to JD, and JD back to MJD

        assert_eq!(time.epoch, 2451545.5 - 2400000.5);
    }


}
