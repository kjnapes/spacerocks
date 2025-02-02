use spacerocks::SpiceKernel;
use spacerocks::Time;
use spacerocks::SpaceRock;


static path_to_de440s: &str = "/Users/thomasruch/Gerdes/de440s.bsp";
static path_to_leap: &str = "/Users/thomasruch/Gerdes/leap_seconds.tls";


// Note on these tests: When running cargo test --test spice_tests, the tests will run by default in parallel. This casues a SPICE error because the kernels are loaded in parallel. 
// To run the tests sequentially, use cargo test --test spice_tests -- --test-threads=1. 
// SpaceRock::from_spice() is commented out as of now because they are more relevant to the SpaceRock struct tests, and because there are a few possible SPICE errors that we will want to
// handle in the future (in progress): 
// 1 - Epoch for a given object is out of bounds of the loaded kernels (SPICE(SPKINSUFFDATA) )
// 2 - The body name is not recognized (SPICE(IDCODENOTFOUND) )
// 3 - The spice file is not loaded (SPICE(NOTRANSLATION) )



#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_spice_kernel_new() {
        let kernel = SpiceKernel::new();
        assert!(kernel.loaded_files.is_empty());
    }

    #[test]
    fn test_spice_kernel_load() {
        let mut kernel = SpiceKernel::new();
        
        // Test successful load
        match kernel.load(path_to_de440s) {
            Ok(()) => assert_eq!(kernel.loaded_files.len(), 1),
            Err(_) => assert!(false, "Failed to load valid kernel"),
        }

        // Test duplicate load
        match kernel.load(path_to_de440s) {
            Ok(()) => assert!(false, "Should not allow duplicate loads"),
            Err(e) => assert!(e.contains("already been loaded")),
        }
    }

    #[test]
    fn test_spice_kernel_unload() {
        let mut kernel = SpiceKernel::new();
        
        // Load some kernels
        kernel.load(path_to_de440s).unwrap();
        kernel.load(path_to_leap).unwrap();
        assert_eq!(kernel.loaded_files.len(), 2);

        // Test unload
        kernel.unload();
        assert!(kernel.loaded_files.is_empty());
    }

    #[test]
    fn test_spice_kernel_display() {
        let mut kernel = SpiceKernel::new();
        
        // Load kernel and verify display
        kernel.load(path_to_de440s).unwrap();
        // Note: display() prints to stdout, primarily test that it doesn't panic
        kernel.display();
    }

    // #[test]
    // fn test_spacerock_from_spice() {
    //     let mut kernel = SpiceKernel::new();
    //     kernel.load(path_to_de440s).unwrap();
    //     kernel.load(path_to_leap).unwrap();

    //     // let epoch = Time::from_fuzzy_str("2460109 utc jd").unwrap();
    //     let epoch = Time::now();

    //     // Test valid creation
    //     match SpaceRock::from_spice("MARS BARYCENTER", &epoch, "ECLIPJ2000", "SSB") {
    //         Ok(rock) => {
    //             assert_eq!(rock.name, "MARS BARYCENTER");
    //             assert_eq!(rock.reference_plane.as_str(), "ECLIPJ2000");
    //             assert_eq!(rock.origin.to_string(), "SSB");
    //             // Note: exact position values would depend on loaded kernels
    //             assert!(rock.position.norm() > 0.0);
    //             assert!(rock.velocity.norm() > 0.0);
    //         },
    //         Err(_) => assert!(false, "Failed to create valid SpaceRock"),
    //     }

    //     // Test invalid body
    //     match SpaceRock::from_spice("INVALID_BODY", &epoch, "ECLIPJ2000", "SSB") {
    //         Ok(_) => assert!(false, "Should not create SpaceRock from invalid body"),
    //         Err(_) => assert!(true),
    //     }

    //     // Test invalid frame
    //     match SpaceRock::from_spice("MARS BARYCENTER", &epoch, "INVALID_FRAME", "SSB") {
    //         Ok(_) => assert!(false, "Should not accept invalid reference frame"),
    //         Err(_) => assert!(true),
    //     }

    //     // Cleanup
    //     kernel.unload();
    // }

    // #[test]
    // fn test_spacerock_from_spice_time_conversion() {
    //     let mut kernel = SpiceKernel::new();
    //     kernel.load(path_to_de440s).unwrap();
    //     kernel.load(path_to_leap).unwrap();

    //     // Test with different time scales
    //     // let utc_time = Time::from_fuzzy_str("2460109 utc jd").unwrap();
    //     // let tdb_time = Time::from_fuzzy_str("2460109 tdb jd").unwrap();

    //     let utc_time = Time::now();
    //     let tdb_time = Time::new(2460682.22, "tdb", "jd").unwrap();

    //     let rock_utc = SpaceRock::from_spice("MARS", &utc_time, "ECLIPJ2000", "SSB").unwrap();
    //     let rock_tdb = SpaceRock::from_spice("MARS", &tdb_time, "ECLIPJ2000", "SSB").unwrap();

    //     // Positions should be slightly different due to time scale differences
    //     assert!((rock_utc.position - rock_tdb.position).norm() > 0.0);

    //     kernel.unload();
    // }

    #[test]
    fn test_spice_kernel_default() {
        let kernel: SpiceKernel = Default::default();
        assert!(kernel.loaded_files.is_empty());
    }
}