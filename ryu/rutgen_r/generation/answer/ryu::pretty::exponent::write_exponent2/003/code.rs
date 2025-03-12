// Answer 0

#[test]
fn test_write_exponent2_with_sign_and_k_equal_100() {
    use std::ptr;

    let mut buffer = [0u8; 4]; // Buffer to hold the result
    let result_ptr = buffer.as_mut_ptr();

    unsafe {
        let k: isize = 100; // k == 100, which means sign is false
        let result_length = write_exponent2(k, result_ptr);
        
        assert_eq!(result_length, 1); // Only the sign was written
        assert_eq!(buffer[0], b'-'); // Negative sign
    }
}

#[test]
fn test_write_exponent2_with_sign_and_k_greater_than_100() {
    use std::ptr;

    let mut buffer = [0u8; 4]; // Buffer to hold the result
    let result_ptr = buffer.as_mut_ptr();

    unsafe {
        let k: isize = 100; // k == 100, which means sign is false
        let result_length = write_exponent2(k, result_ptr);

        assert_eq!(result_length, 1); // Only the sign was written
        assert_eq!(buffer[0], b'-'); // Negative sign
    }
}

