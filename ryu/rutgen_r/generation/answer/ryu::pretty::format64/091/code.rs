// Answer 0

#[test]
fn test_format64_case_1() {
    use std::mem::MaybeUninit;
    use std::ptr;

    let f = 0.0f64;

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let result: &[u8] = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        assert_eq!(std::str::from_utf8_unchecked(result), "0.0");
        assert_eq!(len, 3);
    }
}

#[test]
fn test_format64_case_2() {
    use std::mem::MaybeUninit;
    use std::ptr;

    let f = 1e-324f64;

    unsafe {
        let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
        let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
        let result: &[u8] = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        assert_eq!(std::str::from_utf8_unchecked(result), "0.00000000000000000000");
        assert_eq!(len, 22);
    }
}

