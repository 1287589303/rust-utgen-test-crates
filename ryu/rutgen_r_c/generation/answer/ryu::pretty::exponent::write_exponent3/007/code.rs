// Answer 0

#[test]
fn test_write_exponent3_positive_single_digit() {
    let mut result: [u8; 4] = [0; 4]; // Buffer of sufficient size
    let k: isize = 7; // k is positive and < 10
    let result_ptr = result.as_mut_ptr();

    unsafe {
        let return_value = write_exponent3(k, result_ptr);
        assert_eq!(return_value, 1); // sign as usize + 1 = 0 + 1
        assert_eq!(&result[..], &[b'7', 0, 0, 0]); // Result should be b'7'
    }
}

#[test]
fn test_write_exponent3_positive_two_digit() {
    let mut result: [u8; 4] = [0; 4]; // Buffer of sufficient size
    let k: isize = 45; // k is positive and < 100
    let result_ptr = result.as_mut_ptr();

    unsafe {
        let return_value = write_exponent3(k, result_ptr);
        assert_eq!(return_value, 2); // sign as usize + 1 = 0 + 2
        assert_eq!(&result[..], &[b'4', b'5', 0, 0]); // Result should be b'45'
    }
}

