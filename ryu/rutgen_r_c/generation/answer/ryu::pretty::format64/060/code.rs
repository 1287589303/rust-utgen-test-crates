// Answer 0

#[test]
fn test_format64_positive_nonzero() {
    let f: f64 = 1.234;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let len = unsafe { format64(f, buffer.as_mut_ptr() as *mut u8) };
    let slice = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len) };
    let print = std::str::from_utf8_unchecked(slice);
    assert_eq!(print, "1.234");
    assert_eq!(len, 6); // sign: 0 + 3 (3 for "1.234")
}

#[test]
fn test_format64_large_mantissa() {
    let f: f64 = 1234567890123456789.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let len = unsafe { format64(f, buffer.as_mut_ptr() as *mut u8) };
    let slice = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len) };
    let print = std::str::from_utf8_unchecked(slice);
    assert!(print.contains("e")); // should be in scientific notation
    assert!(len > 3); // sign: 0 + 3 (3 for number part)
}

#[test]
fn test_format64_small_nonzero() {
    let f: f64 = 0.0001234;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let len = unsafe { format64(f, buffer.as_mut_ptr() as *mut u8) };
    let slice = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len) };
    let print = std::str::from_utf8_unchecked(slice);
    assert_eq!(print, "0.0001234");
    assert_eq!(len, 10); // sign: 0 + 3 (0.0001234)
} 

#[test]
fn test_format64_exponent() {
    let f: f64 = 1e30; // large number to invoke scientific notation
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let len = unsafe { format64(f, buffer.as_mut_ptr() as *mut u8) };
    let slice = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len) };
    let print = std::str::from_utf8_unchecked(slice);
    assert!(print.contains("e")); // should be in scientific notation
    assert!(len > 3); // sign: 0 + 3 (for number part)
} 

#[test]
fn test_format64_negative_number() {
    let f: f64 = -1.234;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 24];
    let len = unsafe { format64(f, buffer.as_mut_ptr() as *mut u8) };
    let slice = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len) };
    let print = std::str::from_utf8_unchecked(slice);
    assert_eq!(print, "-1.234");
    assert_eq!(len, 7); // sign: 1 + 3 for "1.234"
}

