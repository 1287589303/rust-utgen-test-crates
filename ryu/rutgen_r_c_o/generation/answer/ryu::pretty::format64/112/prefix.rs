// Answer 0

#[test]
pub unsafe fn test_format64_subnormal() {
    let f: f64 = 5e-324; // Smallest positive subnormal number
    let mut result = [0u8; 24];
    let len = format64(f, result.as_mut_ptr() as *mut u8);
}

#[test]
pub unsafe fn test_format64_subnormal_middle() {
    let f: f64 = 1e-324; // Another small subnormal number
    let mut result = [0u8; 24];
    let len = format64(f, result.as_mut_ptr() as *mut u8);
}

#[test]
pub unsafe fn test_format64_subnormal_large() {
    let f: f64 = 1.5e-324; // Larger subnormal value
    let mut result = [0u8; 24];
    let len = format64(f, result.as_mut_ptr() as *mut u8);
}

