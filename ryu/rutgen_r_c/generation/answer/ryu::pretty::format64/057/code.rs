// Answer 0

#[test]
fn test_format64_negative_case() {
    use std::mem::MaybeUninit;
    use std::slice;
    use std::str;

    let f: f64 = -1e-324;  // This will ensure the sign is true, ieee_exponent == 0, and k == -324
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    
    unsafe {
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        // The expected numeric output here for -1e-324 is "-0.0"
        assert_eq!(print, "-0.0");
    }
}

#[test]
fn test_format64_another_negative_case() {
    use std::mem::MaybeUninit;
    use std::slice;
    use std::str;

    let f: f64 = -5e-324;  // Maintains sign true, ieee_exponent == 0, and k == -324
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];

    unsafe {
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        // The expected numeric output here for -5e-324 is "-0.0000000000000000000000005"
        assert_eq!(print, "-0.0000000000000000000000005");
    }
}

#[test]
fn test_format64_not_a_number_case() {
    use std::mem::MaybeUninit;
    use std::slice;
    use std::str;

    let f: f64 = -0.0;  // This allows to check a regular zero case with sign true, ieee_exponent == 0
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    
    unsafe {
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        // The expected output for -0.0 should be the same as "-0.0"
        assert_eq!(print, "-0.0");
    }
}

