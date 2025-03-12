// Answer 0

#[test]
fn test_format32_zero() {
    use std::{mem::MaybeUninit, slice, str};

    unsafe {
        let f = 0.0f32;
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0");
    }
}

#[test]
fn test_format32_negative_small() {
    use std::{mem::MaybeUninit, slice, str};

    unsafe {
        let f = -0.001234f32;
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.001234");
    }
}

#[test]
fn test_format32_exponent_zero() {
    use std::{mem::MaybeUninit, slice, str};

    unsafe {
        let f = 1e-6f32; // This will match -6 < kk <= 0
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.000001");
    }
}

