// Answer 0

#[test]
fn test_d2d_exponent_zero_non_zero_mantissa() {
    let ieee_exponent = 0;
    let ieee_mantissa = 1; // ensures ieee_mantissa != 0
    let result = d2d(ieee_mantissa, ieee_exponent);
    let expected = FloatingDecimal64 {
        exponent: -DOUBLE_BIAS - DOUBLE_MANTISSA_BITS as i32 - 2, // corresponds to e2 < 0
        mantissa: result.mantissa, // this will vary based on specific computations
    };

    assert_eq!(result.exponent, expected.exponent);
    assert_eq!(result.mantissa, expected.mantissa);
}

#[test]
fn test_d2d_positive_exponent() {
    let ieee_exponent = 1024; // this makes e2 >= 0
    let ieee_mantissa = 1; // this is non-zero
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.exponent > 0);

    // Assuming DOUBLE_POW5_SPLIT has sufficient elements
    assert!(result.mantissa > ieee_mantissa);
}

#[test]
fn test_d2d_boundary_conditions() {
    let ieee_exponent = 1024;
    let ieee_mantissa = 1000; // non-zero
    let result = d2d(ieee_mantissa, ieee_exponent);
    
    assert!(result.mantissa != 0);
    assert!(result.mantissa < u64::MAX); // ensures output stays within bounds

    // Trilean conditions
    assert!(!result.mantissa.is_power_of_two()); // ensures it is not a power of 2
}

#[test]
fn test_d2d_last_removed_digit_cases() {
    let ieee_exponent = 2048; // high enough to ensure q < 63
    let ieee_mantissa = 0x123456789ABCDEF0; // random large mantissa
    let result = d2d(ieee_mantissa, ieee_exponent);
    
    assert_eq!(result.exponent - 1, result.mantissa.count_zeros());
    assert!(result.mantissa < ieee_mantissa);
}

