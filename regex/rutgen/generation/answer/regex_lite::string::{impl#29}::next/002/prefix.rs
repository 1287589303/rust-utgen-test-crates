// Answer 0

#[test]
fn test_sub_capture_matches_next_valid_case() {
    struct TestCaptureLocations;

    impl TestCaptureLocations {
        fn new() -> Self {
            TestCaptureLocations
        }

        fn get(&self, i: usize) -> Option<(usize, usize)> {
            match i {
                0 => Some((0, 5)),
                1 => Some((6, 11)),
                _ => None,
            }
        }
    }

    let haystack = "hello world";
    let slots = TestCaptureLocations::new();
    let pikevm = Arc::new(PikeVM::new());

    let caps = Captures {
        haystack,
        slots,
        pikevm,
    };

    let capture_names = vec![Some(Arc::from("group1")), Some(Arc::from("group2"))];
    let it = capture_names.iter().enumerate();
    let mut sub_capture_matches = SubCaptureMatches {
        caps: &caps,
        it,
    };

    let result = sub_capture_matches.next();
    let expected_group_index = 0;

    // Call the function
    let _ = result;
}

#[test]
fn test_sub_capture_matches_next_with_multiple_groups() {
    struct TestCaptureLocations;

    impl TestCaptureLocations {
        fn new() -> Self {
            TestCaptureLocations
        }

        fn get(&self, i: usize) -> Option<(usize, usize)> {
            match i {
                0 => Some((0, 5)),
                1 => Some((6, 11)),
                2 => Some((12, 17)),
                _ => None,
            }
        }
    }

    let haystack = "hello world again";
    let slots = TestCaptureLocations::new();
    let pikevm = Arc::new(PikeVM::new());

    let caps = Captures {
        haystack,
        slots,
        pikevm,
    };

    let capture_names = vec![
        Some(Arc::from("group1")),
        Some(Arc::from("group2")),
        Some(Arc::from("group3")),
    ];
    let it = capture_names.iter().enumerate();
    let mut sub_capture_matches = SubCaptureMatches {
        caps: &caps,
        it,
    };

    let result = sub_capture_matches.next();

    // Call the function
    let _ = result;
}

#[test]
fn test_sub_capture_matches_next_invalid_index() {
    struct TestCaptureLocations;

    impl TestCaptureLocations {
        fn new() -> Self {
            TestCaptureLocations
        }

        fn get(&self, i: usize) -> Option<(usize, usize)> {
            match i {
                0 => Some((0, 5)),
                _ => None,
            }
        }
    }

    let haystack = "hello";
    let slots = TestCaptureLocations::new();
    let pikevm = Arc::new(PikeVM::new());

    let caps = Captures {
        haystack,
        slots,
        pikevm,
    };

    let capture_names = vec![Some(Arc::from("group1"))];
    let it = capture_names.iter().enumerate();
    let mut sub_capture_matches = SubCaptureMatches {
        caps: &caps,
        it,
    };

    let _ = sub_capture_matches.next(); // valid call
}

