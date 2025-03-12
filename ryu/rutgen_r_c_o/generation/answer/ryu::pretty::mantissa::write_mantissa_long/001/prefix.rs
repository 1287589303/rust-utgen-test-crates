// Answer 0

#[test]
fn test_write_mantissa_long_boundary_high() {
    let output: u64 = 18_446_744_073_709_551_615;
    let mut result = [0u8; 20];
    unsafe {
        write_mantissa_long(output, result.as_mut_ptr().add(19));
    }
}

#[test]
fn test_write_mantissa_long_boundary_low() {
    let output: u64 = 1_000_000_000;
    let mut result = [0u8; 20];
    unsafe {
        write_mantissa_long(output, result.as_mut_ptr().add(19));
    }
}

#[test]
fn test_write_mantissa_long_mid_range() {
    let output: u64 = 10_000_000_000;
    let mut result = [0u8; 20];
    unsafe {
        write_mantissa_long(output, result.as_mut_ptr().add(19));
    }
}

#[test]
fn test_write_mantissa_long_random_value() {
    let output: u64 = 9_765_432_100;
    let mut result = [0u8; 20];
    unsafe {
        write_mantissa_long(output, result.as_mut_ptr().add(19));
    }
}

#[test]
fn test_write_mantissa_long_largest_non_overflow() {
    let output: u64 = 1_000_000_001;
    let mut result = [0u8; 20];
    unsafe {
        write_mantissa_long(output, result.as_mut_ptr().add(19));
    }
}

