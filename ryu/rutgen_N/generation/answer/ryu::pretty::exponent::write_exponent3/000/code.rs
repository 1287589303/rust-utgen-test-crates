// Answer 0

#[test]
fn test_write_exponent3_positive_two_digit() {
    let mut result = [0u8; 4]; // 1 sign + 2 digits + null terminator
    let size = unsafe { write_exponent3(123, result.as_mut_ptr()) };
    assert_eq!(size, 3);
    assert_eq!(&result[..size], b"123");
}

#[test]
fn test_write_exponent3_positive_one_digit() {
    let mut result = [0u8; 4]; // 1 sign + 2 digits + null terminator
    let size = unsafe { write_exponent3(5, result.as_mut_ptr()) };
    assert_eq!(size, 1);
    assert_eq!(&result[..size], b"5");
}

#[test]
fn test_write_exponent3_negative_two_digit() {
    let mut result = [0u8; 4]; // 1 sign + 2 digits + null terminator
    let size = unsafe { write_exponent3(-45, result.as_mut_ptr()) };
    assert_eq!(size, 3);
    assert_eq!(&result[..size], b"-45");
}

#[test]
fn test_write_exponent3_negative_one_digit() {
    let mut result = [0u8; 4]; // 1 sign + 2 digits + null terminator
    let size = unsafe { write_exponent3(-7, result.as_mut_ptr()) };
    assert_eq!(size, 2);
    assert_eq!(&result[..size], b"-7");
}

#[test]
fn test_write_exponent3_zero() {
    let mut result = [0u8; 4]; // 1 sign + 2 digits + null terminator
    let size = unsafe { write_exponent3(0, result.as_mut_ptr()) };
    assert_eq!(size, 1);
    assert_eq!(&result[..size], b"0");
}

