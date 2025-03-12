// Answer 0

#[test]
unsafe fn test_format64_zero() {
    let f = 0.0f64;
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let _len = format64(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
unsafe fn test_format64_large_exponent() {
    let f = 1.0e300f64; // This value will result in k < -324
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let _len = format64(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
unsafe fn test_format64_negative_zero() {
    let f = -0.0f64; // Sign is false, ieee_exponent == 0
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let _len = format64(f, buffer.as_mut_ptr() as *mut u8);
}

