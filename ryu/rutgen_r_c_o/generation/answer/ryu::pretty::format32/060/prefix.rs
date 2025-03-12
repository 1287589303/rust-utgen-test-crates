// Answer 0

#[test]
pub unsafe fn test_format32_positive_nonzero_float() {
    let f: f32 = 1234.5678;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
pub unsafe fn test_format32_negative_nonzero_float() {
    let f: f32 = -9876.5432;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
pub unsafe fn test_format32_small_positive_float() {
    let f: f32 = 0.0001234;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
pub unsafe fn test_format32_small_negative_float() {
    let f: f32 = -0.0005678;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
pub unsafe fn test_format32_large_positive_float() {
    let f: f32 = 1.234e30;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
pub unsafe fn test_format32_large_negative_float() {
    let f: f32 = -9.876e30;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    let len = format32(f, buffer.as_mut_ptr() as *mut u8);
}

