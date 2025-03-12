// Answer 0

#[test]
fn test_capture_locations_get_valid_index_0() {
    use core::num::NonZeroUsize;

    struct MockCaptureLocations {
        data: Vec<Option<NonMaxUsize>>,
    }

    impl CaptureLocations {
        fn new(data: Vec<Option<NonMaxUsize>>) -> Self {
            CaptureLocations(data)
        }
    }

    let locs = MockCaptureLocations {
        data: vec![
            NonMaxUsize::new(1), // start for index 0
            NonMaxUsize::new(18), // end for index 0
            NonMaxUsize::new(7), // start for index 1
            NonMaxUsize::new(18), // end for index 1
        ].into_iter().collect(),
    };

    let result = locs.get(0);
}

#[test]
fn test_capture_locations_get_valid_index_1() {
    use core::num::NonZeroUsize;

    struct MockCaptureLocations {
        data: Vec<Option<NonMaxUsize>>,
    }

    impl CaptureLocations {
        fn new(data: Vec<Option<NonMaxUsize>>) -> Self {
            CaptureLocations(data)
        }
    }

    let locs = MockCaptureLocations {
        data: vec![
            NonMaxUsize::new(1),
            NonMaxUsize::new(18),
            NonMaxUsize::new(7),
            NonMaxUsize::new(18),
        ].into_iter().collect(),
    };

    let result = locs.get(1);
}

#[test]
fn test_capture_locations_get_valid_index_2() {
    use core::num::NonZeroUsize;

    struct MockCaptureLocations {
        data: Vec<Option<NonMaxUsize>>,
    }

    impl CaptureLocations {
        fn new(data: Vec<Option<NonMaxUsize>>) -> Self {
            CaptureLocations(data)
        }
    }

    let locs = MockCaptureLocations {
        data: vec![
            NonMaxUsize::new(1),
            NonMaxUsize::new(18),
            NonMaxUsize::new(7),
            NonMaxUsize::new(18),
        ].into_iter().collect(),
    };

    let result = locs.get(2);
}

