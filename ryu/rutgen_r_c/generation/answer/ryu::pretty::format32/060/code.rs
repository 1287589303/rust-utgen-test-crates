// Answer 0

#[test]
fn test_format32_positive_non_zero() {
    use std::mem::MaybeUninit;
    use std::slice;
    use std::str;

    let f = 1.234f32; // setting a positive non-zero float
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "1.234");
        assert_eq!(len, 5); // sign (0) + 3 for the string length
    }
}

#[test]
fn test_format32_large_positive() {
    use std::mem::MaybeUninit;
    use std::slice;
    use std::str;

    let f = 12345678.0f32; // large positive float
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "12345678.0");
        assert_eq!(len, 10); // sign (0) + 3 for the string length
    }
}

#[test]
fn test_format32_small_positive() {
    use std::mem::MaybeUninit;
    use std::slice;
    use std::str;

    let f = 0.0001234f32; // small positive float
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0001234");
        assert_eq!(len, 11); // sign (0) + 3 for the string length
    }
}

#[test]
fn test_format32_exponential_notation() {
    use std::mem::MaybeUninit;
    use std::slice;
    use std::str;

    let f = 1.234e30f32; // float that results in exponential notation
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "1.234e30");
        assert_eq!(len, 10); // sign (0) + 3 for the string length
    }
}

