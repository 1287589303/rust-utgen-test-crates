// Answer 0

#[test]
fn test_get_with_invalid_index_too_large() {
    let locs = CaptureLocations(vec![Some(NonMaxUsize::new(1).unwrap()), Some(NonMaxUsize::new(5).unwrap())]);
    let result = locs.get(2); // 2 * 2 = 4, which is out of bounds for the vec
}

#[test]
fn test_get_with_invalid_index_zero() {
    let locs = CaptureLocations(vec![]);
    let result = locs.get(0); // 0 * 2 = 0, which is out of bounds for the vec
}

#[test]
fn test_get_with_invalid_index_negative() {
    let locs = CaptureLocations(vec![Some(NonMaxUsize::new(3).unwrap())]);
    let result = locs.get(usize::MAX); // usize::MAX * 2 would overflow
}

