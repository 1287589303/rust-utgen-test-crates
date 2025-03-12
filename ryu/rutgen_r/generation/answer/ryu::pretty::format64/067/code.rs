// Answer 0

#[test]
fn test_format64_non_zero_exponent_large_mantissa() {
    use std::{mem::MaybeUninit, slice, str};

    let f = 1e30_f64; // This will ensure ieee_exponent is non-zero and ieee_mantissa is non-zero.
    
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "1e30");
    }
}

#[test]
fn test_format64_small_exponent_negative() {
    use std::{mem::MaybeUninit, slice, str};

    let f = 0.00001234_f64; // This will ensure the mantissa has a value and small exponent.
    
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.00001234");
    }
} 

#[test]
fn test_format64_zero_exponent() {
    use std::{mem::MaybeUninit, slice, str};

    let f = 1234567890.0; // This ensures there's a long mantissa and a significant exponent, as well as a value under 10^10.
    
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "1234567890");
    }
} 

#[test]
fn test_format64_negative_exponent_extreme() {
    use std::{mem::MaybeUninit, slice, str};

    let f = 1e-30; // This ensures a large negative exponent while keeping the mantissa non-zero.
    
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.000000000000000000000001");
    }
} 

#[test]
fn test_format64_small_number() {
    use std::{mem::MaybeUninit, slice, str};

    let f = 3.6e-4; // This ensures we handle small numbers with a mantissa that results in leading zeros.
    
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.00036");
    }
} 

#[test]
fn test_format64_negative_small_number() {
    use std::{mem::MaybeUninit, slice, str};

    let f = -4.5e-5; // This tests negative small numbers with a non-zero mantissa.
    
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.000045");
    }
} 

#[test]
fn test_format64_large_negative_exponent() {
    use std::{mem::MaybeUninit, slice, str};

    let f = 1e-324; // Tests the smallest positive number greater than zero.
    
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001");
    }
} 

#[test]
fn test_format64_positive_integer() {
    use std::{mem::MaybeUninit, slice, str};

    let f = 2.0; // Tests a straightforward positive integer case.
    
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "2");
    }
} 

