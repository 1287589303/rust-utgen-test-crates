// Answer 0

#[test]
fn test_get_valid_capture_group() {
    // Create a CaptureLocations with valid NonMaxUsize values
    let locs = CaptureLocations(vec![
        Some(NonMaxUsize::new(5).unwrap()), // start
        Some(NonMaxUsize::new(10).unwrap()), // end
    ]);
    
    // This should be valid, as i = 0 is within the bounds
    let _ = locs.get(0);
}

#[test]
fn test_get_boundary_capture_group() {
    // Create a CaptureLocations with valid NonMaxUsize values
    let locs = CaptureLocations(vec![
        Some(NonMaxUsize::new(5).unwrap()), // start
        Some(NonMaxUsize::new(10).unwrap()), // end
    ]);
    
    // Check for out-of-bounds case
    let _ = locs.get(1);
}

#[test]
fn test_get_no_capture_group() {
    // Create a CaptureLocations with invalid NonMaxUsize values
    let locs = CaptureLocations(vec![
        Some(NonMaxUsize::new(5).unwrap()), // start
        Some(NonMaxUsize::new(10).unwrap()), // end
    ]);
    
    // Test the case where slot.checked_add(1) is None
    let _ = locs.get(2);
}

