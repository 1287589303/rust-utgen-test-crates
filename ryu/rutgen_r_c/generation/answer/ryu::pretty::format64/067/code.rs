// Answer 0

#[test]
unsafe fn test_format64_positive_number() {
    let mut buffer = [0u8; 24];
    let f = 1.5f64;

    let len = format64(f, buffer.as_mut_ptr());

    assert_eq!(len, 4); // Expected output length for "1.5" should match
    assert_eq!(std::str::from_utf8_unchecked(&buffer[..len]), "1.5");
}

#[test]
unsafe fn test_format64_exponent_positive() {
    let mut buffer = [0u8; 24];
    let f = 1.234e10f64; // This will have exponent > 16

    let len = format64(f, buffer.as_mut_ptr());

    assert!(len > 0); // It should have written something
    assert!(std::str::from_utf8_unchecked(&buffer[..len]).contains("e"));
}

#[test]
unsafe fn test_format64_exponent_negative() {
    let mut buffer = [0u8; 24];
    let f = 1.234e-6f64; // This will lead to 0.000001234

    let len = format64(f, buffer.as_mut_ptr());

    assert_eq!(std::str::from_utf8_unchecked(&buffer[..len]), "0.000001234");
}

#[test]
unsafe fn test_format64_single_digit_exponent() {
    let mut buffer = [0u8; 24];
    let f = 9.0f64; // This should yield "9"

    let len = format64(f, buffer.as_mut_ptr());

    assert_eq!(len, 1);
    assert_eq!(std::str::from_utf8_unchecked(&buffer[..len]), "9");
}

#[test]
unsafe fn test_format64_boundary_case_k() {
    let mut buffer = [0u8; 24];
    let f = 1.0e-324; // This will use k = -324

    let len = format64(f, buffer.as_mut_ptr());

    assert!(len > 0); // It should have written something
    assert_eq!(std::str::from_utf8_unchecked(&buffer[..len]).starts_with("0."));
}

#[test]
unsafe fn test_format64_length_one_case() {
    let mut buffer = [0u8; 24];
    let f = 2.0f64; // This should yield length == 1 case

    let len = format64(f, buffer.as_mut_ptr());

    assert_eq!(len, 2);
    assert_eq!(std::str::from_utf8_unchecked(&buffer[..len]), "2e0");
}

