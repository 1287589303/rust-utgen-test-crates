// Answer 0

#[test]
fn test_d2d_case_1() {
    let ieee_mantissa: u64 = 0x8000000000000; // Non-zero mantissa
    let ieee_exponent: u32 = 0x400; // Non-zero exponent, ensuring e2 >= 0

    let result = d2d(ieee_mantissa, ieee_exponent);

    assert!(result.exponent < 0); // Verify e2 >= 0 is false
    // Additional assertions based on the calculated result can be added here
}

#[test]
fn test_d2d_case_2() {
    let ieee_mantissa: u64 = 0x8000000000000; // Non-zero mantissa
    let ieee_exponent: u32 = 0x300; // Non-zero exponent, ensuring e2 >= 0

    let result = d2d(ieee_mantissa, ieee_exponent);
    
    assert!(result.exponent > 0); // Now e2 should be greater than 0
    // Additional assertions based on the calculated result can be added here
}

#[test]
fn test_d2d_case_3() {
    let ieee_mantissa: u64 = 0x1FFFFF; // A non-zero mantissa ensuring multiple rounds of division
    let ieee_exponent: u32 = 0x800; // Non-zero exponent

    let result = d2d(ieee_mantissa, ieee_exponent);

    assert!(result.mantissa != 0); // Ensure mantissa is calculated correctly
    assert!(result.exponent > 0); // It should still return a valid exponent
}

#[test]
fn test_d2d_case_4() {
    let ieee_mantissa: u64 = 0x1FFFFF; // Non-zero mantissa
    let ieee_exponent: u32 = 0x900; // Non-zero exponent

    let result = d2d(ieee_mantissa, ieee_exponent);

    assert!(result.exponent == 63); // Ensure we are testing the max bounds
}

#[test]
fn test_d2d_case_5() {
    let ieee_mantissa: u64 = 0xFFFFFFFFFFFFFFFF; // Using maximum value for mantissa
    let ieee_exponent: u32 = 0x800; // Non-zero exponent

    let result = d2d(ieee_mantissa, ieee_exponent);
  
    assert!(result.mantissa != 0); 
    assert!(result.mantissa != result.exponent); // They should differ
}

