// Answer 0

#[test]
fn test_format32_positive_nonzero() {
    let f: f32 = 1.23456;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    
    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "1.23456");
    }
}

#[test]
fn test_format32_large_positive() {
    let f: f32 = 123456789.0;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    
    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "123456789.0");
    }
}

#[test]
fn test_format32_small_positive() {
    let f: f32 = 0.0001234;
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    
    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.0001234");
    }
}

#[test]
fn test_format32_exponential_notation() {
    let f: f32 = 1234567.0; // Should produce scientific notation
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    
    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "1.234567e6");
    }
}

#[test]
fn test_format32_boundary_condition() {
    let f: f32 = 1e-45; // Just above underflow threshold
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 16];
    
    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
        let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
        let print = std::str::from_utf8_unchecked(slice);
        assert_eq!(print, "0.000000000000000000000000000000000000000000000000000001");
    }
}

