// Answer 0

#[test]
unsafe fn test_format64_case_zero() {
    let f: f64 = 0.0;
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let len = format64(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
unsafe fn test_format64_case_positive() {
    let f: f64 = 0.0; // Testing with 0.0
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let len = format64(f, buffer.as_mut_ptr() as *mut u8);
}

