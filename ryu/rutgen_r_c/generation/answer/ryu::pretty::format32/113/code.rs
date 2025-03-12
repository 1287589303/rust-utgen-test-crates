// Answer 0

#[test]
fn test_format32_case_zero_float() {
    let mut buffer = [0u8; 16];
    let f: f32 = 0.0;

    unsafe {
        let len = ryu::format32(f, buffer.as_mut_ptr());
        let formatted = core::str::from_utf8_unchecked(&buffer[0..len]);
        assert_eq!(formatted, "0.0");
    }
}

#[test]
fn test_format32_case_negative_zero_float() {
    let mut buffer = [0u8; 16];
    let f: f32 = -0.0;

    unsafe {
        let len = ryu::format32(f, buffer.as_mut_ptr());
        let formatted = core::str::from_utf8_unchecked(&buffer[0..len]);
        assert_eq!(formatted, "-0.0");
    }
}

#[test]
fn test_format32_case_min_float() {
    let mut buffer = [0u8; 16];
    let f: f32 = -1.0e-45;

    unsafe {
        let len = ryu::format32(f, buffer.as_mut_ptr());
        let formatted = core::str::from_utf8_unchecked(&buffer[0..len]);
        assert_eq!(formatted, "-0.000000000000000000000000000001"); // Special case handling, long representation
    }
}

#[test]
fn test_format32_case_small_float() {
    let mut buffer = [0u8; 16];
    let f: f32 = 1.0e-40;

    unsafe {
        let len = ryu::format32(f, buffer.as_mut_ptr());
        let formatted = core::str::from_utf8_unchecked(&buffer[0..len]);
        assert_eq!(formatted, "0.00000000000000000000000000001");
    }
}

