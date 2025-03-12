// Answer 0

#[test]
unsafe fn test_format32_negative_smallest() {
    let f: f32 = -1e-45;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
unsafe fn test_format32_negative_boundary() {
    let f: f32 = -1e-6;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
unsafe fn test_format32_negative_mid_range() {
    let f: f32 = -5e-20;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
}

