// Answer 0

#[test]
fn test_write_mantissa_zero() {
    let mut buffer = [0u8; 10];
    let output: u32 = 0;
    let result = buffer.as_mut_ptr().offset(buffer.len() as isize);
    unsafe {
        write_mantissa(output, result);
    }
}

#[test]
fn test_write_mantissa_one() {
    let mut buffer = [0u8; 10];
    let output: u32 = 1;
    let result = buffer.as_mut_ptr().offset(buffer.len() as isize);
    unsafe {
        write_mantissa(output, result);
    }
}

#[test]
fn test_write_mantissa_nine() {
    let mut buffer = [0u8; 10];
    let output: u32 = 9;
    let result = buffer.as_mut_ptr().offset(buffer.len() as isize);
    unsafe {
        write_mantissa(output, result);
    }
}

#[test]
fn test_write_mantissa_ten() {
    let mut buffer = [0u8; 10];
    let output: u32 = 10;
    let result = buffer.as_mut_ptr().offset(buffer.len() as isize);
    unsafe {
        write_mantissa(output, result);
    }
}

#[test]
fn test_write_mantissa_99() {
    let mut buffer = [0u8; 10];
    let output: u32 = 99;
    let result = buffer.as_mut_ptr().offset(buffer.len() as isize);
    unsafe {
        write_mantissa(output, result);
    }
}

