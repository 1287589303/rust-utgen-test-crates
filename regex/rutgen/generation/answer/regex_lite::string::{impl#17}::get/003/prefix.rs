// Answer 0

#[test]
fn test_capture_locations_valid_index() {
    let valid_start = NonMaxUsize::new(1).unwrap();
    let valid_end = NonMaxUsize::new(3).unwrap();
    let locs = CaptureLocations(vec![Some(valid_start), Some(valid_end)]);
    assert!(locs.get(0).is_some());
}

#[test]
fn test_capture_locations_valid_index_with_none() {
    let valid_start = NonMaxUsize::new(1).unwrap();
    let locs = CaptureLocations(vec![Some(valid_start), None]);
    assert!(locs.get(0).is_some());
}

#[test]
fn test_capture_locations_invalid_index() {
    let invalid_start = NonMaxUsize::new(1).unwrap();
    let locs = CaptureLocations(vec![Some(invalid_start)]);
    assert!(locs.get(1).is_none());
}

