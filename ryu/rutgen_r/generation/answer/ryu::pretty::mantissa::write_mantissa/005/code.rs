// Answer 0

#[test]
fn test_write_mantissa_case_1() {
    let output: u32 = 100; // output >= 100 is true
    let mut result = [0u8; 10]; // buffer to hold the result

    unsafe {
        write_mantissa(output, result.as_mut_ptr().add(9)); // initializing result pointer
    }

    assert_eq!(&result[8..10], b"00"); // DigitTable value for 100 is "00"
}

#[test]
fn test_write_mantissa_case_2() {
    let output: u32 = 10; // output >= 10 is true
    let mut result = [0u8; 10]; // buffer to hold the result
    
    unsafe {
        write_mantissa(output, result.as_mut_ptr().add(9)); // initializing result pointer
    }

    assert_eq!(result[9], b'0' + 10); // DigitTable value for 10 is '0' + 10
}

