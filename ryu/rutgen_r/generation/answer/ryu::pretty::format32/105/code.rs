// Answer 0

#[test]
fn test_format32_case1() {
    struct TestStruct;

    let f: f32 = 1e13; // This should satisfy ieee_exponent's requirements.
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];

    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "10000000000000.0");
    }
}

#[test]
fn test_format32_case2() {
    struct TestStruct;

    let f: f32 = 1e12; // This should also satisfy k < 0 and have kk <= 13.
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];

    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "1000000000000.0");
    }
}

#[test]
fn test_format32_case3() {
    struct TestStruct;

    let f: f32 = 1e10; // This will fit within the specified requirements.
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];

    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "10000000000.0");
    }
}

