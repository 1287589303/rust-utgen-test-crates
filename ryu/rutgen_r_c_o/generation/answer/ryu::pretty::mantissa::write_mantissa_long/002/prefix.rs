// Answer 0

#[test]
fn test_write_mantissa_long_zero_output() {
    let mut result = [0u8; 10];
    unsafe {
        write_mantissa_long(0, result.as_mut_ptr().offset(10)); 
    }
}

#[test]
fn test_write_mantissa_long_small_output() {
    let mut result = [0u8; 10];
    unsafe {
        write_mantissa_long(42, result.as_mut_ptr().offset(10)); 
    }
}

#[test]
fn test_write_mantissa_long_large_output() {
    let mut result = [0u8; 10];
    unsafe {
        write_mantissa_long(4_294_967_295, result.as_mut_ptr().offset(10)); 
    }
}

