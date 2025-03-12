// Answer 0

#[test]
unsafe fn test_format64_zero() {
    let f = 0.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
    let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
    let _print = std::str::from_utf8_unchecked(slice);
}

#[test]
unsafe fn test_format64_negative_zero() {
    let f = -0.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let len = ryu::format64(f, buffer.as_mut_ptr() as *mut u8);
    let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
    let _print = std::str::from_utf8_unchecked(slice);
}

