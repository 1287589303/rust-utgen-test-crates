// Answer 0

#[test]
pub unsafe fn test_format64_case1() {
    let f: f64 = 1e1; // ieee_exponent > 0, ieee_mantissa = 0
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let len = format64(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
pub unsafe fn test_format64_case2() {
    let f: f64 = 5e2; // ieee_exponent > 0, ieee_mantissa = 0
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let len = format64(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
pub unsafe fn test_format64_case3() {
    let f: f64 = 9e15; // ieee_exponent > 0, ieee_mantissa = 0
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let len = format64(f, buffer.as_mut_ptr() as *mut u8);
}

