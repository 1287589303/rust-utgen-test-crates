// Answer 0

#[test]
fn test_get_valid_index_with_none_start() {
    struct ValidCaptureLocations {
        locations: CaptureLocations,
    }

    impl ValidCaptureLocations {
        fn new() -> Self {
            let locs = CaptureLocations(vec![
                NonMaxUsize::new(1).unwrap(), // start for group 0
                NonMaxUsize::new(5).unwrap(), // end for group 0
                NonMaxUsize::new(6).unwrap(), // start for group 1
                NonMaxUsize::new(10).unwrap(), // end for group 1
            ]);
            ValidCaptureLocations { locations: locs }
        }
    }

    let locs = ValidCaptureLocations::new();
    
    // This call uses a valid index (0) but expects None for the start since it's moved.
    let _result = locs.locations.get(1);
}

#[test]
fn test_get_index_out_of_range() {
    struct OutOfRangeCaptureLocations {
        locations: CaptureLocations,
    }

    impl OutOfRangeCaptureLocations {
        fn new() -> Self {
            let locs = CaptureLocations(vec![
                NonMaxUsize::new(1).unwrap(), // start for group 0
                NonMaxUsize::new(5).unwrap(), // end for group 0
            ]);
            OutOfRangeCaptureLocations { locations: locs }
        }
    }

    let locs = OutOfRangeCaptureLocations::new();
    
    // This call uses an index (1) that leads to out-of-bounds for get. 
    let _result = locs.locations.get(1);
}

