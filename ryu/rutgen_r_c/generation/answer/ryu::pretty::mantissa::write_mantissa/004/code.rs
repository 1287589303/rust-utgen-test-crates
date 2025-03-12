// Answer 0

#[test]
fn test_write_mantissa_bound_10000() {
    let mut buffer = [0u8; 20];
    let result_ptr = buffer.as_mut_ptr().offset(20); // Pointer to the end of the buffer
    unsafe {
        write_mantissa(10_000, result_ptr);
    }
    assert_eq!(&buffer[18..20], b"10");
}

#[test]
fn test_write_mantissa_less_than_10000() {
    let mut buffer = [0u8; 20];
    let result_ptr = buffer.as_mut_ptr().offset(20);
    unsafe {
        write_mantissa(9999, result_ptr);
    }
    assert_eq!(&buffer[19..20], b'9'); // 9999 should end with '9'
}

#[test]
fn test_write_mantissa_less_than_100() {
    let mut buffer = [0u8; 20];
    let result_ptr = buffer.as_mut_ptr().offset(20);
    unsafe {
        write_mantissa(99, result_ptr);
    }
    assert_eq!(&buffer[18..20], b"99"); // 99 should be written as "99"
}

#[test]
fn test_write_mantissa_less_than_10() {
    let mut buffer = [0u8; 20];
    let result_ptr = buffer.as_mut_ptr().offset(20);
    unsafe {
        write_mantissa(9, result_ptr);
    }
    assert_eq!(&buffer[19..20], b'9'); // 9 should be written as '9'
}

