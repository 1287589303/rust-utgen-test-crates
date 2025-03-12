// Answer 0

#[test]
fn test_write_mantissa_small_value() {
    let mut buffer = [0u8; 20];
    let output: u32 = 5;
    let result_ptr = buffer.as_mut_ptr().add(20);
    
    unsafe {
        write_mantissa(output, result_ptr);
    }
    
    let result = String::from_utf8_lossy(&buffer[16..20]);
    assert_eq!(result.trim_end_matches(char::from(0)), "5");
}

#[test]
fn test_write_mantissa_medium_value() {
    let mut buffer = [0u8; 20];
    let output: u32 = 1234;
    let result_ptr = buffer.as_mut_ptr().add(20);
    
    unsafe {
        write_mantissa(output, result_ptr);
    }
    
    let result = String::from_utf8_lossy(&buffer[16..20]);
    assert_eq!(result.trim_end_matches(char::from(0)), "1234");
}

#[test]
fn test_write_mantissa_large_value() {
    let mut buffer = [0u8; 20];
    let output: u32 = 12345678;
    let result_ptr = buffer.as_mut_ptr().add(20);
    
    unsafe {
        write_mantissa(output, result_ptr);
    }
    
    let result = String::from_utf8_lossy(&buffer[12..20]);
    assert_eq!(result.trim_end_matches(char::from(0)), "12345678");
}

#[test]
fn test_write_mantissa_zero() {
    let mut buffer = [0u8; 20];
    let output: u32 = 0;
    let result_ptr = buffer.as_mut_ptr().add(20);
    
    unsafe {
        write_mantissa(output, result_ptr);
    }
    
    let result = String::from_utf8_lossy(&buffer[16..20]);
    assert_eq!(result.trim_end_matches(char::from(0)), "0");
}

#[test]
fn test_write_mantissa_boundary_case() {
    let mut buffer = [0u8; 20];
    let output: u32 = 10000;
    let result_ptr = buffer.as_mut_ptr().add(20);
    
    unsafe {
        write_mantissa(output, result_ptr);
    }
    
    let result = String::from_utf8_lossy(&buffer[12..20]);
    assert_eq!(result.trim_end_matches(char::from(0)), "10000");
}

