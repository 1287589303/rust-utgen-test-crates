// Answer 0

#[test]
fn test_write_exponent2_positive_single_digit() {
    let mut buffer = [0u8; 2];
    let result = unsafe { write_exponent2(5, buffer.as_mut_ptr()) };
    assert_eq!(result, 1);
    assert_eq!(buffer, [b'5', 0]);
}

#[test]
fn test_write_exponent2_positive_double_digit() {
    let mut buffer = [0u8; 3];
    let result = unsafe { write_exponent2(27, buffer.as_mut_ptr()) };
    assert_eq!(result, 2);
    assert_eq!(buffer, [b'2', b'7', 0]);
}

#[test]
fn test_write_exponent2_negative_single_digit() {
    let mut buffer = [0u8; 3];
    let result = unsafe { write_exponent2(-3, buffer.as_mut_ptr()) };
    assert_eq!(result, 2);
    assert_eq!(buffer, [b'-', b'3', 0]);
}

#[test]
fn test_write_exponent2_negative_double_digit() {
    let mut buffer = [0u8; 3];
    let result = unsafe { write_exponent2(-99, buffer.as_mut_ptr()) };
    assert_eq!(result, 3);
    assert_eq!(buffer, [b'-', b'9', b'9']);
}

#[test]
#[should_panic]
fn test_write_exponent2_out_of_bounds() {
    let mut buffer = [0u8; 2];
    unsafe { write_exponent2(100, buffer.as_mut_ptr()) }; // This should panic due to the debug_assert at line 39
}

