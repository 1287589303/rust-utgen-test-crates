// Answer 0

#[test]
fn test_d2d_case_1() {
    let ieee_mantissa: u64 = 0x0000000000000001; // Non-zero mantissa
    let ieee_exponent: u32 = 0; // This leads to e2 == 0
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.exponent, -1074); // e10 + removed should reflect e2 == 0-> resulting in exponent calculation
    assert_eq!(result.mantissa, 1); // Should be the smallest non-zero mantissa based on input
}

#[test]
fn test_d2d_case_2() {
    let ieee_mantissa: u64 = 0x0000000000000002; // Non-zero mantissa
    let ieee_exponent: u32 = 0; // This leads to e2 == 0
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.exponent, -1074); // Same as case 1 due to exponent calculation
    assert_eq!(result.mantissa, 2); // Should reflect the input mantissa
}

#[test]
fn test_d2d_case_3() {
    let ieee_mantissa: u64 = 0x0000000000000004; // Non-zero mantissa
    let ieee_exponent: u32 = 0; // This leads to e2 == 0
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.exponent, -1074); // Consistent with previous cases
    assert_eq!(result.mantissa, 4); // Should reflect the input mantissa
}

#[test]
fn test_d2d_case_4() {
    let ieee_mantissa: u64 = 0x0000000000000008; // Non-zero mantissa
    let ieee_exponent: u32 = 0; // This leads to e2 == 0
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.exponent, -1074); // Consistent with previous cases
    assert_eq!(result.mantissa, 8); // Should reflect the input mantissa
}

#[test]
fn test_d2d_case_5() {
    let ieee_mantissa: u64 = 0x0000000000000010; // Non-zero mantissa
    let ieee_exponent: u32 = 0; // This leads to e2 == 0
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.exponent, -1074); // Consistent with previous cases
    assert_eq!(result.mantissa, 16); // Should reflect the input mantissa
}

