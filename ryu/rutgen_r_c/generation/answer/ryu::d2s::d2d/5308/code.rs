// Answer 0

#[test]
fn test_d2d_case1() {
    let ieee_mantissa: u64 = 0b0000000000000000000000000000000000000000000000000000000000000001; // example non-zero value
    let ieee_exponent: u32 = 0; // ieee_exponent == 0

    let result = d2d(ieee_mantissa, ieee_exponent);

    assert_eq!(result.exponent, -1074); // Expected exponent value based on the input
    assert_eq!(result.mantissa, 4992); // Expected mantissa value based on the input
}

#[test]
fn test_d2d_case2() {
    let ieee_mantissa: u64 = 0b0000000000000000000000000000000000000000000000000000000000000010; // example non-zero value
    let ieee_exponent: u32 = 0; // ieee_exponent == 0

    let result = d2d(ieee_mantissa, ieee_exponent);

    assert_eq!(result.exponent, -1073); // Adjust expected exponent based on the input
    assert_eq!(result.mantissa, 4993); // Adjust expected mantissa based on the input
}

