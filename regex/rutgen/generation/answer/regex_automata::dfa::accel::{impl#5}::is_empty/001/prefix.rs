// Answer 0

#[test]
fn test_is_empty_with_zero_length() {
    let accel = Accel { bytes: [0, 1, 2, 3, 4, 5, 6, 7] }; // first byte is 0
    let _ = accel.is_empty();
}

#[test]
fn test_is_empty_with_all_zero_bytes() {
    let accel = Accel { bytes: [0; ACCEL_CAP] }; // all bytes are 0
    let _ = accel.is_empty();
}

#[test]
fn test_is_empty_with_non_zero_length() {
    let accel = Accel { bytes: [1, 1, 2, 3, 4, 5, 6, 7] }; // first byte is 1
    let _ = accel.is_empty();
}

#[test]
fn test_is_empty_with_boundary_value_max() {
    let accel = Accel { bytes: [0, 255, 255, 255, 255, 255, 255, 255] }; // first byte is 0, others are max value
    let _ = accel.is_empty();
}

