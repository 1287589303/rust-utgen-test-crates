// Answer 0

#[test]
fn test_format32_positive_zero() {
    let f: f32 = 0.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len: usize;

    unsafe {
        len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
    }

    let slice = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len) };
    let print = std::str::from_utf8_unchecked(slice);
    assert_eq!(print, "0.0");
}

#[test]
fn test_format32_positive_small_number() {
    let f: f32 = 1e-10;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len: usize;

    unsafe {
        len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
    }

    let slice = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len) };
    let print = std::str::from_utf8_unchecked(slice);
    assert_eq!(print, "0.0000000001");
}

#[test]
fn test_format32_large_exponent() {
    let f: f32 = 1e10;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len: usize;

    unsafe {
        len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
    }

    let slice = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len) };
    let print = std::str::from_utf8_unchecked(slice);
    assert_eq!(print, "10000000000.0");
}

