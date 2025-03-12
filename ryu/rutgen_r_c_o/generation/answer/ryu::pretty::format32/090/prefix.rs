// Answer 0

#[test]
unsafe fn test_format32_case_zero() {
    let f: f32 = 0.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
unsafe fn test_format32_case_normalized_min() {
    let f: f32 = 1.0e-45;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
unsafe fn test_format32_case_normalized_max() {
    let f: f32 = 1.0e-44;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
}

