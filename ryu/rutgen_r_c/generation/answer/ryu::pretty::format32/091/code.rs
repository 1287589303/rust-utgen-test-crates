// Answer 0

unsafe fn test_format32_case1() {
    let f: f32 = 0.0f32;
    let mut buffer = [core::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
    let slice = core::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
    let printed = core::str::from_utf8_unchecked(slice);
    assert_eq!(printed, "0.0");
}

unsafe fn test_format32_case2() {
    let f: f32 = 1e-45; // This will ensure k == -45
    let mut buffer = [core::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
    let slice = core::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
    let printed = core::str::from_utf8_unchecked(slice);
    assert_eq!(printed, "0.000000000000000");
}

unsafe fn test_format32_case3() {
    let f: f32 = 1.234e+13; // This will ensure kk == 13
    let mut buffer = [core::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
    let slice = core::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
    let printed = core::str::from_utf8_unchecked(slice);
    assert_eq!(printed, "12340000000000.0");
}

unsafe fn test_format32_case4() {
    let f: f32 = 9.999999e-7; // This will ensure 0 < k <= 0
    let mut buffer = [core::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
    let slice = core::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
    let printed = core::str::from_utf8_unchecked(slice);
    assert_eq!(printed, "0.00000099999999");
}

