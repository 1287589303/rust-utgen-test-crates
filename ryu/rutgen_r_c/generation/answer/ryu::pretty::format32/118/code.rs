// Answer 0

#[test]
fn test_format32_positive_zero() {
    let f: f32 = 0.0;
    let mut buffer = [0u8; 16];
    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr());
        let result = std::str::from_utf8_unchecked(&buffer[..len]);
        assert_eq!(result, "0.0");
    }
}

#[test]
fn test_format32_min_positive_subnormal() {
    let f: f32 = std::f32::MIN_POSITIVE; // This represents the smallest positive subnormal number
    let mut buffer = [0u8; 16];
    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr());
        let result = std::str::from_utf8_unchecked(&buffer[..len]);
        assert!(result.starts_with("0."));
    }
}

#[test]
fn test_format32_positional_scenario_with_high_exponent() {
    let f: f32 = 1.0e-45; // Testing with an exponent that meets the criteria
    let mut buffer = [0u8; 16];
    unsafe {
        let len = ryu::raw::format32(f, buffer.as_mut_ptr());
        let result = std::str::from_utf8_unchecked(&buffer[..len]);
        assert!(result.starts_with("0."));
    }
}

