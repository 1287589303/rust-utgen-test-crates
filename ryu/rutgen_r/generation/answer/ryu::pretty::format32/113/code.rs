// Answer 0

#[test]
fn test_format32_zero_float() {
    use std::mem::MaybeUninit;
    use std::slice;
    use std::str;

    let f = 0.0f32;
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0");
    }
}

#[test]
fn test_format32_small_positive_float() {
    use std::mem::MaybeUninit;
    use std::slice;
    use std::str;

    let f = 0.001f32; // Ensure this leads to small exponent
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.001");
    }
}

#[test]
fn test_format32_one_point_zero_float() {
    use std::mem::MaybeUninit;
    use std::slice;
    use std::str;

    let f = 1.0f32; // Standard representation
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "1.0");
    }
}

