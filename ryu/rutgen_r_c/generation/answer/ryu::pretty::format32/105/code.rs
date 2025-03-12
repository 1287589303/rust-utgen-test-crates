// Answer 0

#[test]
fn test_format32_normal_case() {
    use std::mem::MaybeUninit;
    use std::slice;
    use std::str;

    let f = 1.234f32; // Use a normal positive float
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "1.234");
    }
}

#[test]
fn test_format32_zero_case() {
    use std::mem::MaybeUninit;
    use std::slice;
    use std::str;

    let f = 0.0f32; // Zero case, should return "0.0"
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0");
    }
}

#[test]
fn test_format32_boundary_case() {
    use std::mem::MaybeUninit;
    use std::slice;
    use std::str;

    let f = 1e10f32; // Large float to test upper boundary
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "10000000000.0");
    }
}

#[test]
fn test_format32_small_float_case() {
    use std::mem::MaybeUninit;
    use std::slice;
    use std::str;

    let f = 0.0001234f32; // Small float to test lower boundary
    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
        let len = ryu::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0001234");
    }
}

