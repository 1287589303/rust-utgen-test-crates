// Answer 0

#[test]
fn test_format64_positive_integer() {
    unsafe {
        let mut buffer = [0u8; 24];
        let len = format64(1234.0, buffer.as_mut_ptr());
        let result = std::str::from_utf8_unchecked(&buffer[..len]);
        assert_eq!(result, "1234");
    }
}

#[test]
fn test_format64_negative_integer() {
    unsafe {
        let mut buffer = [0u8; 24];
        let len = format64(-5678.0, buffer.as_mut_ptr());
        let result = std::str::from_utf8_unchecked(&buffer[..len]);
        assert_eq!(result, "-5678");
    }
}

#[test]
fn test_format64_small_positive_float() {
    unsafe {
        let mut buffer = [0u8; 24];
        let len = format64(0.001234, buffer.as_mut_ptr());
        let result = std::str::from_utf8_unchecked(&buffer[..len]);
        assert_eq!(result, "0.001234");
    }
}

#[test]
fn test_format64_small_negative_float() {
    unsafe {
        let mut buffer = [0u8; 24];
        let len = format64(-0.000567, buffer.as_mut_ptr());
        let result = std::str::from_utf8_unchecked(&buffer[..len]);
        assert_eq!(result, "-0.000567");
    }
}

#[test]
fn test_format64_large_exponent() {
    unsafe {
        let mut buffer = [0u8; 24];
        let len = format64(1.234e30, buffer.as_mut_ptr());
        let result = std::str::from_utf8_unchecked(&buffer[..len]);
        assert_eq!(result, "1.234e30");
    }
}

#[test]
fn test_format64_zero() {
    unsafe {
        let mut buffer = [0u8; 24];
        let len = format64(0.0, buffer.as_mut_ptr());
        let result = std::str::from_utf8_unchecked(&buffer[..len]);
        assert_eq!(result, "0.0");
    }
}

#[test]
fn test_format64_negative_zero() {
    unsafe {
        let mut buffer = [0u8; 24];
        let len = format64(-0.0, buffer.as_mut_ptr());
        let result = std::str::from_utf8_unchecked(&buffer[..len]);
        assert_eq!(result, "-0.0");
    }
}

#[test]
fn test_format64_large_negative_exponent() {
    unsafe {
        let mut buffer = [0u8; 24];
        let len = format64(1e-10, buffer.as_mut_ptr());
        let result = std::str::from_utf8_unchecked(&buffer[..len]);
        assert_eq!(result, "0.0000000001");
    }
}

