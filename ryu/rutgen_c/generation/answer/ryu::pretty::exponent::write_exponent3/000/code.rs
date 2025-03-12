// Answer 0

#[test]
fn test_write_exponent3_positive_less_than_10() {
    let mut buffer = [0u8; 4];
    let result = unsafe { write_exponent3(5, buffer.as_mut_ptr()) };
    assert_eq!(result, 1);
    assert_eq!(&buffer[..1], b"5");
}

#[test]
fn test_write_exponent3_positive_between_10_and_99() {
    let mut buffer = [0u8; 4];
    let result = unsafe { write_exponent3(42, buffer.as_mut_ptr()) };
    assert_eq!(result, 2);
    assert_eq!(&buffer[..2], b"42");
}

#[test]
fn test_write_exponent3_positive_between_100_and_999() {
    let mut buffer = [0u8; 4];
    let result = unsafe { write_exponent3(256, buffer.as_mut_ptr()) };
    assert_eq!(result, 3);
    assert_eq!(&buffer[..3], b"256");
}

#[test]
fn test_write_exponent3_negative_less_than_10() {
    let mut buffer = [0u8; 4];
    let result = unsafe { write_exponent3(-3, buffer.as_mut_ptr()) };
    assert_eq!(result, 2);
    assert_eq!(&buffer[..2], b"-3");
}

#[test]
fn test_write_exponent3_negative_between_10_and_99() {
    let mut buffer = [0u8; 4];
    let result = unsafe { write_exponent3(-45, buffer.as_mut_ptr()) };
    assert_eq!(result, 3);
    assert_eq!(&buffer[..3], b"-45");
}

#[test]
fn test_write_exponent3_negative_between_100_and_999() {
    let mut buffer = [0u8; 4];
    let result = unsafe { write_exponent3(-987, buffer.as_mut_ptr()) };
    assert_eq!(result, 4);
    assert_eq!(&buffer[..4], b"-987");
}

#[test]
#[should_panic]
fn test_write_exponent3_panic_on_large_negative() {
    let mut buffer = [0u8; 4];
    unsafe { write_exponent3(-1000, buffer.as_mut_ptr()) };  // This should cause a panic due to debug_assert
}

#[test]
#[should_panic]
fn test_write_exponent3_panic_on_large_positive() {
    let mut buffer = [0u8; 4];
    unsafe { write_exponent3(1000, buffer.as_mut_ptr()) };  // This should cause a panic due to debug_assert
}

