// Answer 0

#[test]
fn test_write_exponent3_negative_number_k_equals_1000() {
    let mut buffer = [0u8; 4]; // To hold the result
    let k: isize = -1000;
    let result_ptr = buffer.as_mut_ptr();

    let len = unsafe { write_exponent3(k, result_ptr) };

    assert_eq!(len, 4); // Expecting length with sign and 3 digits
    assert_eq!(&buffer[..len as usize], b"-100");
}

#[test]
fn test_write_exponent3_negative_number_k_greater_than_100() {
    let mut buffer = [0u8; 4]; // To hold the result
    let k: isize = -500;
    let result_ptr = buffer.as_mut_ptr();

    let len = unsafe { write_exponent3(k, result_ptr) };

    assert_eq!(len, 3); // Expecting length with sign and 2 digits
    assert_eq!(&buffer[..len as usize], b"-5");
}

#[test]
fn test_write_exponent3_negative_number_k_positive_bound() {
    let mut buffer = [0u8; 4]; // To hold the result
    let k: isize = -10;
    let result_ptr = buffer.as_mut_ptr();

    let len = unsafe { write_exponent3(k, result_ptr) };

    assert_eq!(len, 3); // Expecting length with sign and single digit
    assert_eq!(&buffer[..len as usize], b"-0");
}

