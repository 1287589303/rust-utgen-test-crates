// Answer 0

#[test]
fn test_format32_case_1() {
    let f: f32 = 1.0;
    let mut buffer = unsafe { [std::mem::MaybeUninit::<u8>::uninit(); 16] };
    let len = unsafe {
        ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8)
    };
    let slice = unsafe {
        std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len)
    };
    let print = unsafe { std::str::from_utf8_unchecked(slice) };
    assert_eq!(print, "1.0");
}

#[test]
fn test_format32_case_2() {
    let f: f32 = 12300000.0;
    let mut buffer = unsafe { [std::mem::MaybeUninit::<u8>::uninit(); 16] };
    let len = unsafe {
        ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8)
    };
    let slice = unsafe {
        std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len)
    };
    let print = unsafe { std::str::from_utf8_unchecked(slice) };
    assert_eq!(print, "12300000.0");
}

#[test]
fn test_format32_case_3() {
    let f: f32 = 0.001234;
    let mut buffer = unsafe { [std::mem::MaybeUninit::<u8>::uninit(); 16] };
    let len = unsafe {
        ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8)
    };
    let slice = unsafe {
        std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len)
    };
    let print = unsafe { std::str::from_utf8_unchecked(slice) };
    assert_eq!(print, "0.001234");
}

#[test]
fn test_format32_case_4() {
    let f: f32 = 1234e30;
    let mut buffer = unsafe { [std::mem::MaybeUninit::<u8>::uninit(); 16] };
    let len = unsafe {
        ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8)
    };
    let slice = unsafe {
        std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len)
    };
    let print = unsafe { std::str::from_utf8_unchecked(slice) };
    assert_eq!(print, "1.234e30");
}

#[test]
fn test_format32_case_5() {
    let f: f32 = 0.0001234;
    let mut buffer = unsafe { [std::mem::MaybeUninit::<u8>::uninit(); 16] };
    let len = unsafe {
        ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8)
    };
    let slice = unsafe {
        std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len)
    };
    let print = unsafe { std::str::from_utf8_unchecked(slice) };
    assert_eq!(print, "0.0001234");
}

