// Answer 0

unsafe fn test_format32_case_zero() {
    let f: f32 = 0.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
    let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
    let print = std::str::from_utf8_unchecked(slice);
    assert_eq!(print, "0.0");
    assert_eq!(len, 3);
}

unsafe fn test_format32_case_negative_zero() {
    let f: f32 = -0.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
    let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
    let print = std::str::from_utf8_unchecked(slice);
    assert_eq!(print, "-0.0");
    assert_eq!(len, 4);
}

unsafe fn test_format32_case_small_positive() {
    let f: f32 = 1e-45; // Very small positive value
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
    let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
    let print = std::str::from_utf8_unchecked(slice);
    assert_eq!(print, "0.000000000000000000");
    assert_eq!(len, 19);
}

unsafe fn test_format32_case_edge_case() {
    let f: f32 = 1.0e-45; // Edge case within bounds
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
    let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
    let print = std::str::from_utf8_unchecked(slice);
    assert_eq!(print, "0.000000000000000000");
    assert_eq!(len, 19);
}

