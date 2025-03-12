// Answer 0

#[test]
fn test_format32_zero() {
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(0.0f32, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0");
    }
}

#[test]
fn test_format32_large_exponent() {
    unsafe {
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(1e40f32, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "1e40");
    }
}

