// Answer 0

#[test]
fn test_get_valid_capture_group() {
    let locs = CaptureLocations(vec![
        Some(NonMaxUsize::new(1).unwrap()),
        Some(NonMaxUsize::new(18).unwrap()),
        Some(NonMaxUsize::new(7).unwrap()),
        Some(NonMaxUsize::new(18).unwrap()),
    ]);

    let result = locs.get(0);
}

#[test]
fn test_get_valid_capture_group_boundary() {
    let locs = CaptureLocations(vec![
        Some(NonMaxUsize::new(1).unwrap()),
        Some(NonMaxUsize::new(10).unwrap()),
        Some(NonMaxUsize::new(5).unwrap()),
        Some(NonMaxUsize::new(10).unwrap()),
    ]);

    let result = locs.get(1);
}

#[test]
fn test_get_invalid_capture_group() {
    let locs = CaptureLocations(vec![
        Some(NonMaxUsize::new(1).unwrap()),
        Some(NonMaxUsize::new(10).unwrap()),
        Some(NonMaxUsize::new(5).unwrap()),
    ]);

    let result = locs.get(2);
}

