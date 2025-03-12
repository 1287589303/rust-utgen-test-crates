// Answer 0

#[test]
fn test_invalid_capture_index_zero() {
    let error = BuildError::invalid_capture_index(0);
    let mut output = String::new();
    error.fmt(&mut output);
}

#[test]
fn test_invalid_capture_index_max() {
    let error = BuildError::invalid_capture_index(4294967295);
    let mut output = String::new();
    error.fmt(&mut output);
}

