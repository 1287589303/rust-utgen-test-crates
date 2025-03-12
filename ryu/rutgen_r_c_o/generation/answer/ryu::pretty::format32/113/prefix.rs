// Answer 0

#[test]
pub unsafe fn test_format32_case1() {
    let f = 1e-38f32;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let length = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
pub unsafe fn test_format32_case2() {
    let f = 1e-40f32;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let length = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
pub unsafe fn test_format32_case3() {
    let f = 1e-39f32;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let length = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
}

