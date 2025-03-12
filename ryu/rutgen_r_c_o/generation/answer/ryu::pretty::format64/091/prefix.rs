// Answer 0

#[test]
pub unsafe fn test_format64_case_zero() {
    let f: f64 = 0.0;
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let len = format64(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
pub unsafe fn test_format64_case_small_positive() {
    let f: f64 = 0.5;
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let len = format64(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
pub unsafe fn test_format64_case_near_one() {
    let f: f64 = 0.9999999999999999;
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let len = format64(f, buffer.as_mut_ptr() as *mut u8);
}

