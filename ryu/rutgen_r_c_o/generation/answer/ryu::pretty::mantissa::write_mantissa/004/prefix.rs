// Answer 0

#[test]
fn test_write_mantissa_output_10000() {
    let output: u32 = 10_000;
    let mut result: [u8; 10] = [0; 10];
    unsafe {
        write_mantissa(output, result.as_mut_ptr().add(9));
    }
}

#[test]
fn test_write_mantissa_output_9999() {
    let output: u32 = 9_999;
    let mut result: [u8; 10] = [0; 10];
    unsafe {
        write_mantissa(output, result.as_mut_ptr().add(9));
    }
}

#[test]
fn test_write_mantissa_output_100() {
    let output: u32 = 100;
    let mut result: [u8; 10] = [0; 10];
    unsafe {
        write_mantissa(output, result.as_mut_ptr().add(9));
    }
}

#[test]
fn test_write_mantissa_output_99() {
    let output: u32 = 99;
    let mut result: [u8; 10] = [0; 10];
    unsafe {
        write_mantissa(output, result.as_mut_ptr().add(9));
    }
}

#[test]
fn test_write_mantissa_output_10() {
    let output: u32 = 10;
    let mut result: [u8; 10] = [0; 10];
    unsafe {
        write_mantissa(output, result.as_mut_ptr().add(9));
    }
}

#[test]
fn test_write_mantissa_output_9() {
    let output: u32 = 9;
    let mut result: [u8; 10] = [0; 10];
    unsafe {
        write_mantissa(output, result.as_mut_ptr().add(9));
    }
}

#[test]
fn test_write_mantissa_output_0() {
    let output: u32 = 0;
    let mut result: [u8; 10] = [0; 10];
    unsafe {
        write_mantissa(output, result.as_mut_ptr().add(9));
    }
}

