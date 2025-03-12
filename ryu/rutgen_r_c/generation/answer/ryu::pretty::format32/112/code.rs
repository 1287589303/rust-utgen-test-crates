// Answer 0

#[test]
fn test_format32_case_zero() {
    let f: f32 = 0.0;
    let mut buffer = [0u8; 16];
    unsafe {
        let len = format32(f, buffer.as_mut_ptr());
        assert_eq!(len, 3);
        assert_eq!(std::str::from_utf8_unchecked(&buffer[..len]), "0.0");
    }
}

#[test]
fn test_format32_case_negative_zero() {
    let f: f32 = -0.0;
    let mut buffer = [0u8; 16];
    unsafe {
        let len = format32(f, buffer.as_mut_ptr());
        assert_eq!(len, 4);
        assert_eq!(std::str::from_utf8_unchecked(&buffer[..len]), "-0.0");
    }
}

#[test]
fn test_format32_case_small() {
    let f: f32 = -0.00001234; // ieee_exponent == 0 and ieee_mantissa != 0
    let mut buffer = [0u8; 16];
    unsafe {
        let len = format32(f, buffer.as_mut_ptr());
        assert_eq!(len, 10);
        assert_eq!(std::str::from_utf8_unchecked(&buffer[..len]), "-0.00001234");
    }
}

#[test]
fn test_format32_case_small_positive() {
    let f: f32 = 0.00001234; // ieee_exponent == 0 and ieee_mantissa != 0
    let mut buffer = [0u8; 16];
    unsafe {
        let len = format32(f, buffer.as_mut_ptr());
        assert_eq!(len, 9);
        assert_eq!(std::str::from_utf8_unchecked(&buffer[..len]), "0.00001234");
    }
}

#[test]
fn test_format32_case_negative_finite() {
    let f: f32 = -0.003; // ieee_exponent == 0 and ieee_mantissa != 0
    let mut buffer = [0u8; 16];
    unsafe {
        let len = format32(f, buffer.as_mut_ptr());
        assert_eq!(len, 6);
        assert_eq!(std::str::from_utf8_unchecked(&buffer[..len]), "-0.003");
    }
}

