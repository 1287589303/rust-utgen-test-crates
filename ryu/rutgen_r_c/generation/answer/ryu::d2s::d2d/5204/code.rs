// Answer 0

#[test]
fn test_d2d_case_1() {
    let ieee_mantissa: u64 = 1; // Non-zero mantissa
    let ieee_exponent: u32 = 0; // Exponent is zero
    let result = d2d(ieee_mantissa, ieee_exponent);
    let expected_exp = -DOUBLE_BIAS - DOUBLE_MANTISSA_BITS as i32 - 2; // e2 initialized to 0
    let expected_mantissa = 0; // since this should result in a conversion near zero
    assert_eq!(result.exponent, expected_exp);
    assert_eq!(result.mantissa, expected_mantissa);
}

#[test]
fn test_d2d_case_2() {
    let ieee_mantissa: u64 = 4; // Non-zero mantissa to meet required preconditions
    let ieee_exponent: u32 = 0; // Exponent is zero
    let result = d2d(ieee_mantissa, ieee_exponent);
    let expected_exp = -DOUBLE_BIAS - DOUBLE_MANTISSA_BITS as i32 - 2; // e2 initialized to 0
    let expected_mantissa = 1; // expected mantissa value
    assert_eq!(result.exponent, expected_exp);
    assert_eq!(result.mantissa, expected_mantissa);
}

#[test]
fn test_d2d_case_3() {
    let ieee_mantissa: u64 = 2; // Non-zero mantissa
    let ieee_exponent: u32 = 0; // Exponent is zero
    let result = d2d(ieee_mantissa, ieee_exponent);
    let expected_exp = -DOUBLE_BIAS - DOUBLE_MANTISSA_BITS as i32 - 2; // e2 equals 0
    let expected_mantissa = 1; // expected mantissa value
    assert!(result.exponent == expected_exp);
    assert!(result.mantissa == expected_mantissa);
}

#[test]
fn test_d2d_case_4() {
    let ieee_mantissa: u64 = 8; // another non-zero mantissa
    let ieee_exponent: u32 = 0; // Exponent is still zero
    let result = d2d(ieee_mantissa, ieee_exponent);
    let expected_exp = -DOUBLE_BIAS - DOUBLE_MANTISSA_BITS as i32 - 2; // Initialization to 0
    let expected_mantissa = 2; // expected mantissa when mantissa is 8
    assert_eq!(result.exponent, expected_exp);
    assert_eq!(result.mantissa, expected_mantissa);
}

