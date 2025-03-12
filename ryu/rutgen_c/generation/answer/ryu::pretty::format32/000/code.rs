// Answer 0

#[test]
fn test_format32_positive() {
    let f: f32 = 1.234;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = unsafe { format32(f, buffer.as_mut_ptr() as *mut u8) };
    let slice = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len) };
    let print = std::str::from_utf8(slice).unwrap();
    assert_eq!(print, "1.234");
}

#[test]
fn test_format32_negative() {
    let f: f32 = -4.567;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = unsafe { format32(f, buffer.as_mut_ptr() as *mut u8) };
    let slice = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len) };
    let print = std::str::from_utf8(slice).unwrap();
    assert_eq!(print, "-4.567");
}

#[test]
fn test_format32_zero() {
    let f: f32 = 0.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = unsafe { format32(f, buffer.as_mut_ptr() as *mut u8) };
    let slice = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len) };
    let print = std::str::from_utf8(slice).unwrap();
    assert_eq!(print, "0.0");
}

#[test]
fn test_format32_small_exponent() {
    let f: f32 = 12.34e-4;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = unsafe { format32(f, buffer.as_mut_ptr() as *mut u8) };
    let slice = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len) };
    let print = std::str::from_utf8(slice).unwrap();
    assert_eq!(print, "0.001234");
}

#[test]
fn test_format32_large_exponent() {
    let f: f32 = 1234e30;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = unsafe { format32(f, buffer.as_mut_ptr() as *mut u8) };
    let slice = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len) };
    let print = std::str::from_utf8(slice).unwrap();
    assert_eq!(print, "1.234e33");
}

