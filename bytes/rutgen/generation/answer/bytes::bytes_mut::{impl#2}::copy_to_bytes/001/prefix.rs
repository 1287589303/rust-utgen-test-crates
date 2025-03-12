// Answer 0

#[test]
fn test_copy_to_bytes_zero_length() {
    let mut bytes_mut = BytesMut::new();
    bytes_mut.extend_from_slice(&[1, 2, 3]);
    let result = bytes_mut.copy_to_bytes(0);
}

#[test]
fn test_copy_to_bytes_full_length() {
    let mut bytes_mut = BytesMut::new();
    bytes_mut.extend_from_slice(&[1, 2, 3]);
    let result = bytes_mut.copy_to_bytes(bytes_mut.len());
}

#[test]
fn test_copy_to_bytes_middle_length() {
    let mut bytes_mut = BytesMut::new();
    bytes_mut.extend_from_slice(&[1, 2, 3, 4, 5]);
    let result = bytes_mut.copy_to_bytes(3);
}

