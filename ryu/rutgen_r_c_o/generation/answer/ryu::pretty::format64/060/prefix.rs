// Answer 0

#[test]
unsafe fn test_format64_positive_normal() {
    let f: f64 = 2.0;
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let len = format64(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
unsafe fn test_format64_positive_large() {
    let f: f64 = 123456789.0;
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let len = format64(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
unsafe fn test_format64_positive_small() {
    let f: f64 = 0.1;
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let len = format64(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
unsafe fn test_format64_positive_fractional() {
    let f: f64 = 1.2345;
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let len = format64(f, buffer.as_mut_ptr() as *mut u8);
}

#[test]
unsafe fn test_format64_positive_exponential() {
    let f: f64 = 1.0e30;
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let len = format64(f, buffer.as_mut_ptr() as *mut u8);
}

