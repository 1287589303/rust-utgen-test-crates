// Answer 0

#[test]
fn test_from_bytes_exceeds_length_limit() {
    let bytes: [u8; 4] = [4, 0, 0, 0];
    let result = Accel::from_bytes(bytes);
}

#[test]
fn test_from_bytes_exactly_length_limit() {
    let bytes: [u8; 4] = [4, 1, 2, 3];
    let result = Accel::from_bytes(bytes);
}

