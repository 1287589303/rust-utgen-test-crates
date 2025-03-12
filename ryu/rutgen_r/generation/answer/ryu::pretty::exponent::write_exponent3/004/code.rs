// Answer 0

#[test]
#[should_panic]
fn test_write_exponent3_sign_true_k_equals_1000() {
    use std::ptr;

    const DIGIT_TABLE: &[u8] = b"0123456789";
    let mut buffer = [0u8; 4]; // buffer to hold the result
    let result_ptr = buffer.as_mut_ptr();

    unsafe {
        let k = 1000; // k set to 1000 to trigger the panic on line 13
        write_exponent3(k, result_ptr);
    }
}

#[test]
fn test_write_exponent3_sign_true_k_under_1000() {
    use std::ptr;

    const DIGIT_TABLE: &[u8] = b"0123456789";  
    let mut buffer = [0u8; 4]; // buffer to hold the result
    let result_ptr = buffer.as_mut_ptr();

    unsafe {
        let k = -999; // k negative to ensure sign is true
        let length = write_exponent3(k, result_ptr);

        assert_eq!(length, 4); // expect '-' and three digits
        assert_eq!(buffer[0], b'-'); // first character should be '-'
        assert_eq!(buffer[1], b'9'); // verify first digit
        assert_eq!(buffer[2], b'9'); // verify second digit
        assert_eq!(buffer[3], b'9'); // verify third digit
    }
}

