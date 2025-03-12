// Answer 0

unsafe fn test_format64_special_case_zero() {
    let f: f64 = 0.0; // ieee_exponent == 0 and ieee_mantissa == 0
    let mut buffer = [0u8; 24];
    let len = format64(f, buffer.as_mut_ptr());

    assert_eq!(len, 3);
    assert_eq!(&buffer[0..3], b"0.0");
}

unsafe fn test_format64_with_k_minus_324() {
    let f: f64 = 1e-324; // Smallest positive subnormal number (k = -324)
    let mut buffer = [0u8; 24];
    let len = format64(f, buffer.as_mut_ptr());

    assert_eq!(len, 4);
    assert_eq!(&buffer[0..4], b"0.0");
}

unsafe fn test_format64_with_k_equals_0() {
    let f: f64 = 1.0; // k = 0, exponent does not change
    let mut buffer = [0u8; 24];
    let len = format64(f, buffer.as_mut_ptr());

    assert_eq!(len, 3);
    assert_eq!(&buffer[0..3], b"1.0");
}

unsafe fn test_format64_with_kk_equals_16() {
    let f: f64 = 1.0e16; // This will lead to kk being 16
    let mut buffer = [0u8; 24];
    let len = format64(f, buffer.as_mut_ptr());

    assert_eq!(len, 16);
    assert_eq!(&buffer[0..16], b"1.0000000000000000");
}

unsafe fn test_format64_boundary_with_length_1() {
    let f: f64 = 1e30; // exponents and mantissa lead to condition where length == 1
    let mut buffer = [0u8; 24];
    let len = format64(f, buffer.as_mut_ptr());

    assert_eq!(len, 6); // 2 digits for exponent and 1 digit for the mantissa
    assert_eq!(&buffer[0..6], b"1e30");
}

