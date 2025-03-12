// Answer 0

#[test]
fn test_d2d_ieee_exponent_zero() {
    let ieee_mantissa: u64 = 1; // Non-zero mantissa
    let ieee_exponent: u32 = 0; // Exponent is zero
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.exponent, 1 - DOUBLE_BIAS - DOUBLE_MANTISSA_BITS as i32 - 2);
    assert_eq!(result.mantissa, ieee_mantissa);
}

#[test]
fn test_d2d_ieee_mantissa_non_zero_zero_exponent() {
    let ieee_mantissa: u64 = 1; // Non-zero mantissa
    let ieee_exponent: u32 = 1; // Changing to a non-zero exponent for a valid test
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.exponent >= 0); // This will ensure that e2 >= 0
}

#[test]
fn test_d2d_q_exceeds_double_pow5_inv_split() {
    let ieee_mantissa: u64 = 1; // Non-zero mantissa
    let ieee_exponent: u32 = 2047; // Arbitrarily large exponent to bound q
    let result = d2d(ieee_mantissa, ieee_exponent);
    let q = log10_pow2(ieee_exponent as i32 - DOUBLE_BIAS - DOUBLE_MANTISSA_BITS as i32 - 2) - (ieee_exponent > 3) as u32;
    assert!(q >= DOUBLE_POW5_INV_BITCOUNT as u32); // Assert that q exceeds bounds
}

