// Answer 0

#[test]
unsafe fn test_format64_negative_zero() {
    let f = -0.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
    let _ = len; // Using len to trigger function execution
}

#[test]
unsafe fn test_format64_min_negative_value() {
    let f = -1e-324;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
    let _ = len; // Using len to trigger function execution
}

