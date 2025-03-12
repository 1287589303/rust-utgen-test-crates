// Answer 0

#[test]
fn test_write_exponent2_positive_single_digit() {
    let mut buffer = [0u8; 3];
    let result = unsafe { write_exponent2(5, buffer.as_mut_ptr()) };
    assert_eq!(result, 1);
    assert_eq!(buffer[0], b'5');
}

#[test]
fn test_write_exponent2_zero() {
    let mut buffer = [0u8; 3];
    let result = unsafe { write_exponent2(0, buffer.as_mut_ptr()) };
    assert_eq!(result, 1);
    assert_eq!(buffer[0], b'0');
}

