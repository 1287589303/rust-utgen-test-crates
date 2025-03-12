// Answer 0

#[test]
fn test_write_mantissa_long_zero_output() {
    let mut buffer = [0u8; 20];
    let result_ptr = buffer.as_mut_ptr().add(20); // Pointing to the end of the buffer
    unsafe {
        write_mantissa_long(0, result_ptr);
    }
    assert_eq!(&buffer[18..20], b"00"); // Check for the '00' result in the buffer
}

#[test]
fn test_write_mantissa_long_smaller_than_threshold() {
    let mut buffer = [0u8; 20];
    let result_ptr = buffer.as_mut_ptr().add(20); // Pointing to the end of the buffer
    unsafe {
        write_mantissa_long(100, result_ptr);
    }
    assert_eq!(&buffer[19..20], b'0'); // Check for '0' in the buffer
} 

#[test]
fn test_write_mantissa_long_edge_case() {
    let mut buffer = [0u8; 20];
    let result_ptr = buffer.as_mut_ptr().add(20); // Pointing to the end of the buffer
    unsafe {
        write_mantissa_long(9, result_ptr);
    }
    assert_eq!(&buffer[19..20], b'9'); // Check for '9' in the buffer
}

