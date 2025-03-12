// Answer 0

#[test]
pub unsafe fn test_format32_zero() {
    let f: f32 = 0.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
pub unsafe fn test_format32_negative_zero() {
    let f: f32 = -0.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
}

