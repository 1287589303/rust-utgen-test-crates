// Answer 0

#[test]
fn test_invalid_byte_zero_index_zero_byte() {
    let error = DecodeError::InvalidByte(0, 0);
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_invalid_byte_zero_index_max_byte() {
    let error = DecodeError::InvalidByte(0, 255);
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_invalid_byte_max_index_zero_byte() {
    let error = DecodeError::InvalidByte(usize::MAX, 0);
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_invalid_byte_max_index_max_byte() {
    let error = DecodeError::InvalidByte(usize::MAX, 255);
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

