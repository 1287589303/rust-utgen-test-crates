// Answer 0

#[test]
fn test_read_version_version_mismatch() {
    let slice = [1, 0, 0, 0];
    let expected_version = 0;
    let _ = read_version(&slice, expected_version);
}

#[test]
fn test_read_version_version_mismatch_2() {
    let slice = [1, 0, 0, 0];
    let expected_version = 2;
    let _ = read_version(&slice, expected_version);
}

#[test]
fn test_read_version_buffer_too_small() {
    let slice: [u8; 3] = [0; 3];
    let expected_version = 0;
    let _ = read_version(&slice, expected_version);
}

#[test]
fn test_read_version_success() {
    let slice = [1, 0, 0, 0];
    let expected_version = 1;
    let _ = read_version(&slice, expected_version);
}

#[test]
fn test_read_version_expected_version_zero() {
    let slice = [0, 0, 0, 0];
    let expected_version = 0;
    let _ = read_version(&slice, expected_version);
}

#[test]
fn test_read_version_expected_version_two() {
    let slice = [0, 0, 0, 0];
    let expected_version = 2;
    let _ = read_version(&slice, expected_version);
}

#[test]
fn test_read_version_unexpected_value() {
    let slice = [2, 0, 0, 0];
    let expected_version = 1;
    let _ = read_version(&slice, expected_version);
}

