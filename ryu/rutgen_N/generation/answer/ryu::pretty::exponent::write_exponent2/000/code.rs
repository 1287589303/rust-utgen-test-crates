// Answer 0

#[test]
fn test_write_exponent2_positive_single_digit() {
    let mut buffer = [0u8; 3]; // Enough space for '0' + '\0'
    let result = unsafe { write_exponent2(5, buffer.as_mut_ptr()) };
    assert_eq!(result, 1); // 1 character written
    assert_eq!(&buffer[..result], b"5"); // Expected output is "5"
}

#[test]
fn test_write_exponent2_positive_double_digit() {
    let mut buffer = [0u8; 3]; // Enough space for "10" + '\0'
    let result = unsafe { write_exponent2(10, buffer.as_mut_ptr()) };
    assert_eq!(result, 2); // 2 characters written
    assert_eq!(&buffer[..result], b"10"); // Expected output is "10"
}

#[test]
fn test_write_exponent2_negative_single_digit() {
    let mut buffer = [0u8; 4]; // Enough space for "-5" + '\0'
    let result = unsafe { write_exponent2(-5, buffer.as_mut_ptr()) };
    assert_eq!(result, 2); // 2 characters written (including '-')
    assert_eq!(&buffer[..result], b"-5"); // Expected output is "-5"
}

#[test]
fn test_write_exponent2_negative_double_digit() {
    let mut buffer = [0u8; 4]; // Enough space for "-10" + '\0'
    let result = unsafe { write_exponent2(-10, buffer.as_mut_ptr()) };
    assert_eq!(result, 3); // 3 characters written (including '-')
    assert_eq!(&buffer[..result], b"-10"); // Expected output is "-10"
}

#[test]
fn test_write_exponent2_zero() {
    let mut buffer = [0u8; 3]; // Enough space for '0' + '\0'
    let result = unsafe { write_exponent2(0, buffer.as_mut_ptr()) };
    assert_eq!(result, 1); // 1 character written
    assert_eq!(&buffer[..result], b"0"); // Expected output is "0"
}

