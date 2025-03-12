// Answer 0

#[test]
fn test_write_mantissa_below_ten_thousand() {
    let mut buffer = [0u8; 32];
    let output: u32 = 9999; // output < 10_000
    let result_ptr = buffer.as_mut_ptr().offset(buffer.len() as isize);
    unsafe {
        write_mantissa(output, result_ptr);
    }
    let result_str = std::str::from_utf8(&buffer[(buffer.len() - 1)..]).unwrap();
    assert_eq!(result_str, "9999");
}

#[test]
fn test_write_mantissa_below_one_hundred() {
    let mut buffer = [0u8; 32];
    let output: u32 = 99; // output < 100
    let result_ptr = buffer.as_mut_ptr().offset(buffer.len() as isize);
    unsafe {
        write_mantissa(output, result_ptr);
    }
    let result_str = std::str::from_utf8(&buffer[(buffer.len() - 1)..]).unwrap();
    assert_eq!(result_str, "99");
}

#[test]
fn test_write_mantissa_below_ten() {
    let mut buffer = [0u8; 32];
    let output: u32 = 5; // output < 10
    let result_ptr = buffer.as_mut_ptr().offset(buffer.len() as isize);
    unsafe {
        write_mantissa(output, result_ptr);
    }
    let result_str = std::str::from_utf8(&buffer[(buffer.len() - 1)..]).unwrap();
    assert_eq!(result_str, "5");
}

