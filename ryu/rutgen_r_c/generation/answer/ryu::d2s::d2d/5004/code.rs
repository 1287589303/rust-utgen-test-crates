// Answer 0

#[test]
fn test_d2d_ieee_exponent_zero_ieee_mantissa_non_zero() {
    let ieee_mantissa: u64 = 1; // non-zero mantissa
    let ieee_exponent: u32 = 0; // exponent is zero

    let result = d2d(ieee_mantissa, ieee_exponent);

    assert_eq!(result.exponent, -1071); // e2 should be 0, thus exp = e10 + removed where removed is 0
    assert_ne!(result.mantissa, 0); // Ensure mantissa is non-zero
}

#[test]
fn test_d2d_negative_e2_and_k_exceeds_bounds() {
    let ieee_mantissa: u64 = 4; // Sample mantissa
    let ieee_exponent: u32 = 3; // Causes e2 to be negative

    let result = d2d(ieee_mantissa, ieee_exponent);

    assert_eq!(result.exponent, -1071); // Adjust based on calculation
    assert_ne!(result.mantissa, 0); // Ensure mantissa is non-zero
}

#[test]
fn test_d2d_conditions_with_powers() {
    let ieee_mantissa: u64 = 8; // Choose a mantissa causing q == 21
    let ieee_exponent: u32 = 1234; // Example exponent leading to valid state

    let result = d2d(ieee_mantissa, ieee_exponent);

    assert_ne!(result.mantissa, 0); // Ensure output is valid
    assert_ne!(result.exponent, 0); // Should evaluate to a valid exponent
}

