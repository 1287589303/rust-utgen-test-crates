// Answer 0

#[test]
fn test_format64_case_with_zero() {
    let f: f64 = 0.0;
    let mut buffer = [0u8; 24];
    unsafe {
        let len = ryu::format64(f, buffer.as_mut_ptr());
        assert_eq!(len, 3); // Expecting "0.0", which has a length of 3
        assert_eq!(std::str::from_utf8_unchecked(&buffer[..len]), "0.0");
    }
}

#[test]
fn test_format64_case_with_zero_negative() {
    let f: f64 = -0.0;
    let mut buffer = [0u8; 24];
    unsafe {
        let len = ryu::format64(f, buffer.as_mut_ptr());
        assert_eq!(len, 4); // Expecting "-0.0", which has a length of 4
        assert_eq!(std::str::from_utf8_unchecked(&buffer[..len]), "-0.0");
    }
}

#[test]
fn test_format64_case_with_small_exponent() {
    let f: f64 = 1e-324; // IEEE exponent == 0 (i.e., subnormal)
    let mut buffer = [0u8; 24];
    unsafe {
        let len = ryu::format64(f, buffer.as_mut_ptr());
        assert_eq!(len, 7); // Expecting "0.0000001", which has a length of 7
        assert_eq!(std::str::from_utf8_unchecked(&buffer[..len]), "0.0000001");
    }
}

