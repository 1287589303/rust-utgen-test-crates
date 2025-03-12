// Answer 0

#[test]
fn test_write_exponent2_positive_single_digit() {
    let mut buffer = [0u8; 3]; // We need an extra byte for potential null termination
    let result_ptr = buffer.as_mut_ptr();

    unsafe {
        let k: isize = 5; // k is positive and less than 10
        let return_value = write_exponent2(k, result_ptr);

        assert_eq!(return_value, 1); // sign is false, so return value is 1
        assert_eq!(buffer[0], b'5'); // The correct character for digit 5
        assert_eq!(buffer[1], 0); // The second byte should remain 0
    }
}

#[test]
fn test_write_exponent2_positive_two_digit() {
    let mut buffer = [0u8; 3]; // We need an extra byte for potential null termination
    let result_ptr = buffer.as_mut_ptr();

    unsafe {
        let k: isize = 10; // k is positive and exactly 10, meaning it should be a two-digit case
        let return_value = write_exponent2(k, result_ptr);

        assert_eq!(return_value, 2); // sign is false, and return value should be 2
        assert_eq!(buffer[0], b'1'); // First digit should be '1'
        assert_eq!(buffer[1], b'0'); // Second digit should be '0'
        assert_eq!(buffer[2], 0); // The third byte should remain 0
    }
}

