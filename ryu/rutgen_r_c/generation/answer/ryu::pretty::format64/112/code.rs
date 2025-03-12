// Answer 0

unsafe fn test_format64_case1() {
    let f: f64 = 0.0; // Sign is false, ieee_exponent == 0
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    
    let len = format64(f, buffer.as_mut_ptr() as *mut u8);
    
    assert_eq!(len, 3);
    
    let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
    let result = std::str::from_utf8_unchecked(slice);

    assert_eq!(result, "0.0");
}

unsafe fn test_format64_case2() {
    let f: f64 = 1e-324; // k is -324
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];

    let len = format64(f, buffer.as_mut_ptr() as *mut u8);

    assert!(len > 0); // Check if something was written

    let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
    let result = std::str::from_utf8_unchecked(slice);

    assert!(result.len() > 0); // Check if the result is a valid string
}

unsafe fn test_format64_case3() {
    let f: f64 = 0.001234; // To check k < 0 and kk == 0
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];

    let len = format64(f, buffer.as_mut_ptr() as *mut u8);

    assert!(len > 0);
    
    let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
    let result = std::str::from_utf8_unchecked(slice);
    
    assert_eq!(result, "0.001234");
}

unsafe fn test_format64_case4() {
    let f: f64 = 9e-324; // k < 0, 0 < kk <= 0
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];

    let len = format64(f, buffer.as_mut_ptr() as *mut u8);

    assert!(len > 0);
    
    let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
    let result = std::str::from_utf8_unchecked(slice);
    
    assert_eq!(result, "0.0000000000000000000000000009");
}

unsafe fn test_format64_case5() {
    let f: f64 = 10e-324; // To check k < 0, kk == 1
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];

    let len = format64(f, buffer.as_mut_ptr() as *mut u8);

    assert!(len > 0);
    
    let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
    let result = std::str::from_utf8_unchecked(slice);
    
    assert_eq!(result, "0.000000000000000000000000001"); // Verify the output
}

#[test]
unsafe fn run_tests() {
    test_format64_case1();
    test_format64_case2();
    test_format64_case3();
    test_format64_case4();
    test_format64_case5();
}

