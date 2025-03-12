// Answer 0

#[test]
fn test_format32_negative_zero() {
    unsafe {
        let f: f32 = -0.0;
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.0");
    }
}

#[test]
fn test_format32_negative_small() {
    unsafe {
        let f: f32 = -1e-45;
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.0000000000000000000123e-44");
    }
}

#[test]
fn test_format32_negative_exponent() {
    unsafe {
        let f: f32 = -1.23e-6;
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.00000123");
    }
}

#[test]
fn test_format32_negative_large() {
    unsafe {
        let f: f32 = -1.0e30; 
        let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-1e30");
    }
}

