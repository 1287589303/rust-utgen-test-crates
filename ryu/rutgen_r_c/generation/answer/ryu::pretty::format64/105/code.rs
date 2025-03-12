// Answer 0

#[test]
fn test_format64_case1() {
    let f: f64 = 0.0; // This sets ieee_exponent == 0 and ieee_mantissa == 0
    let mut buffer = [0u8; 24];
    let len = unsafe { format64(f, buffer.as_mut_ptr() as *mut u8) };
    let result = std::str::from_utf8(&buffer[..len]).unwrap();
    assert_eq!(result, "0.0");
}

#[test]
fn test_format64_case2() {
    let f: f64 = 1e-324; // This ensures 0 < kk and kk <= 16
    let mut buffer = [0u8; 24];
    let len = unsafe { format64(f, buffer.as_mut_ptr() as *mut u8) };
    let result = std::str::from_utf8(&buffer[..len]).unwrap();
    assert_eq!(result, "1e-324");
}

