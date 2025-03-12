// Answer 0

#[test]
fn test_write_mantissa_case_1() {
    let output: u32 = 9999;
    let mut result = [0u8; 20];
    let result_ptr = result.as_mut_ptr().add(19); // Start at the end of the buffer

    unsafe {
        write_mantissa(output, result_ptr);
    }

    assert_eq!(&result[15..19], b"9999"); // Ensure we get the correct string representation
}

#[test]
fn test_write_mantissa_case_2() {
    let output: u32 = 100;
    let mut result = [0u8; 20];
    let result_ptr = result.as_mut_ptr().add(19); // Start at the end of the buffer

    unsafe {
        write_mantissa(output, result_ptr);
    }

    assert_eq!(&result[17..19], b"00"); // Ensure last two characters are "00"
    assert_eq!(result[19], b'1'); // Ensure it starts with '1'
}

#[test]
fn test_write_mantissa_case_3() {
    let output: u32 = 10;
    let mut result = [0u8; 20];
    let result_ptr = result.as_mut_ptr().add(19); // Start at the end of the buffer

    unsafe {
        write_mantissa(output, result_ptr);
    }

    assert_eq!(result[18], b'0'); // Ensure that the last character at offset -1 is '0'
    assert_eq!(result[19], b'0'); // Ensure the second to last character is also '0'
}

