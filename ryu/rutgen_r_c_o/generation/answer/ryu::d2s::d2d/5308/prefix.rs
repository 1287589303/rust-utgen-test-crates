// Answer 0

#[test]
fn test_d2d_case_1() {
    let ieee_mantissa: u64 = 1; // ieee_mantissa != 0
    let ieee_exponent: u32 = 0; // ieee_exponent == 0
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case_2() {
    let ieee_mantissa: u64 = 8; // ieee_mantissa != 0
    let ieee_exponent: u32 = 0; // ieee_exponent == 0
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case_3() {
    let ieee_mantissa: u64 = 16; // ieee_mantissa != 0
    let ieee_exponent: u32 = 0; // ieee_exponent == 0
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case_4() {
    let ieee_mantissa: u64 = 24; // ieee_mantissa != 0
    let ieee_exponent: u32 = 0; // ieee_exponent == 0
    let result = d2d(ieee_mantissa, ieee_exponent);
}

