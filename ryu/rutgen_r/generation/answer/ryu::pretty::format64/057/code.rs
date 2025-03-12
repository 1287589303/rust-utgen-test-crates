// Answer 0

#[test]
fn test_format64_negative_zero() {
    use std::{mem::MaybeUninit, slice, str};

    let f = -0.0f64; // sign is true, ieee_exponent == 0 and ieee_mantissa == 0

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.0");
    }
}

#[test]
fn test_format64_negative_small_value() {
    use std::{mem::MaybeUninit, slice, str};

    let f = -1.0e-324f64; // sign is true, ieee_exponent == 0, ieee_mantissa is non-zero

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "-1e-324");
    }
}

#[test]
fn test_format64_negative_large_exponent() {
    use std::{mem::MaybeUninit, slice, str};

    let f = -1.0e30; // sign is true, ieee_exponent is larger than 0

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "-1e30");
    }
}

