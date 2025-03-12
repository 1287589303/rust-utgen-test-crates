// Answer 0

#[test]
fn test_format64_zero() {
    use std::mem::MaybeUninit;
    use std::ptr;

    let f = 0.0f64; // Precondition: ieee_exponent == 0 and ieee_mantissa == 0
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];

    unsafe {
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0"); // Confirming the formatted output
        assert_eq!(len, 3); // Precondition guaranteed: sign is false, ieee_exponent == 0, etc.
    }
}

#[test]
fn test_format64_negative_small() {
    use std::mem::MaybeUninit;
    use std::ptr;

    let f = -0.0001234f64; // A small negative number to meet the conditions
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];

    unsafe {
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-0.0001234"); // Confirming the formatted output
    }
}

#[test]
fn test_format64_large_exponent() {
    use std::mem::MaybeUninit;
    use std::ptr;

    let f = 1.0e30f64; // A large number with a significant exponent 
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];

    unsafe {
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "1e30"); // Confirming the formatted output for large exponent
    }
}

#[test]
fn test_format64_negative_large_exponent() {
    use std::mem::MaybeUninit;
    use std::ptr;

    let f = -1.0e30f64; // A large negative number
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];

    unsafe {
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "-1e30"); // Confirming the formatted output for large negative exponent
    }
}

