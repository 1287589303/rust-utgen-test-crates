// Answer 0

#[test]
fn test_add_valid_byte_under_capacity() {
    let mut accel = Accel::new();
    assert!(accel.add(b'a')); // Add first byte
    assert!(accel.add(b'b')); // Add second byte
}

#[test]
fn test_add_byte_is_space() {
    let mut accel = Accel::new();
    assert!(accel.add(b'a')); // Add first byte
    assert!(accel.add(b'b')); // Add second byte
    let result = accel.add(b' '); // Attempt to add space byte
    assert!(!result); // Should return false since byte is space
}

#[test]
fn test_add_duplicate_byte() {
    let mut accel = Accel::new();
    assert!(accel.add(b'a')); // Add first byte
    assert!(accel.add(b'b')); // Add second byte
    let _ = accel.add(b'c'); // Add third byte
    let result = std::panic::catch_unwind(|| accel.add(b'a')); // Attempt to add duplicate byte
    assert!(result.is_err()); // Should panic
}

