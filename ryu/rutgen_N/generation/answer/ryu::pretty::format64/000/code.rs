// Answer 0

#[test]
fn test_format64_positive() {
    use std::{mem::MaybeUninit, slice, str};

    let f = 1.234f64;

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "1.234");
    }
}

#[test]
fn test_format64_zero() {
    use std::{mem::MaybeUninit, slice, str};

    let f = 0.0f64;

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0");
    }
}

#[test]
fn test_format64_negative() {
    use std::{mem::MaybeUninit, slice, str};

    let f = -1.234f64;

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "-1.234");
    }
}

#[test]
fn test_format64_large_number() {
    use std::{mem::MaybeUninit, slice, str};

    let f = 1e30f64;

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "1e30");
    }
}

#[test]
fn test_format64_small_number() {
    use std::{mem::MaybeUninit, slice, str};

    let f = 1e-30f64;

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.000000000000000000000001");
    }
}

#[test]
fn test_format64_boundary() {
    use std::{mem::MaybeUninit, slice, str};

    let f = 1e-324f64;

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0");
    }
}

#[test]
#[should_panic]
fn test_format64_nan() {
    use std::{mem::MaybeUninit, slice, str};

    let f = f64::NAN;

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8); // Expected to panic due to NaN
    }
}

#[test]
#[should_panic]
fn test_format64_infinity() {
    use std::{mem::MaybeUninit, slice, str};

    let f = f64::INFINITY;

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8); // Expected to panic due to infinity
    }
}

