// Answer 0

#[test]
fn test_get_valid_capture_group() {
    use alloc::vec::Vec;
    use crate::int::NonMaxUsize;
    use crate::CaptureLocations;

    let capture_vec = vec![
        NonMaxUsize::new(1).unwrap(), // Start of group 0
        NonMaxUsize::new(18).unwrap(), // End of group 0
        NonMaxUsize::new(1).unwrap(), // Start of group 1
        NonMaxUsize::new(6).unwrap(), // End of group 1
    ];

    let locs = CaptureLocations(capture_vec);
    let result = locs.get(0);
}

#[test]
fn test_get_valid_capture_group_1() {
    use alloc::vec::Vec;
    use crate::int::NonMaxUsize;
    use crate::CaptureLocations;

    let capture_vec = vec![
        NonMaxUsize::new(1).unwrap(), // Start of group 0
        NonMaxUsize::new(18).unwrap(), // End of group 0
        NonMaxUsize::new(1).unwrap(), // Start of group 1
        NonMaxUsize::new(6).unwrap(), // End of group 1
    ];

    let locs = CaptureLocations(capture_vec);
    let result = locs.get(1);
}

#[test]
fn test_get_no_capture_groups() {
    use alloc::vec::Vec;
    use crate::int::NonMaxUsize;
    use crate::CaptureLocations;

    let capture_vec = vec![];
    let locs = CaptureLocations(capture_vec);
    let result = locs.get(0);
}

