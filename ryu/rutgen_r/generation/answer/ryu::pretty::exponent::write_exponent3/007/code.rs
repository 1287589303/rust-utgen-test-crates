// Answer 0

#[test]
fn test_write_exponent3_case_k_le_9() {
    use std::ptr;

    let mut result: [u8; 4] = [0; 4]; // buffer to hold the result
    let result_ptr = result.as_mut_ptr();

    unsafe {
        let k: isize = 9; // k < 10
        let return_value = write_exponent3(k, result_ptr);
        assert_eq!(return_value, 1); // sign as usize + 1, sign is false
        assert_eq!(result[0], b'0' + k as u8); // expected result is '9'
    }
}

#[test]
fn test_write_exponent3_case_k_eq_0() {
    use std::ptr;

    let mut result: [u8; 4] = [0; 4]; // buffer to hold the result
    let result_ptr = result.as_mut_ptr();

    unsafe {
        let k: isize = 0; // k < 10
        let return_value = write_exponent3(k, result_ptr);
        assert_eq!(return_value, 1); // sign as usize + 1, sign is false
        assert_eq!(result[0], b'0' + k as u8); // expected result is '0'
    }
}

