// Answer 0

#[test]
fn test_invalid_capture_index_zero() {
    let error = BuildError::invalid_capture_index(0);
}

#[test]
fn test_invalid_capture_index_max() {
    let error = BuildError::invalid_capture_index(u32::MAX);
}

#[test]
fn test_invalid_capture_index_middle() {
    let error = BuildError::invalid_capture_index(123456);
}

#[test]
fn test_invalid_capture_index_large_value() {
    let error = BuildError::invalid_capture_index(1_000_000);
}

