// Answer 0

#[test]
fn test_d2d_case1() {
    let ieee_mantissa: u64 = 0; // iee_mantissa = 0
    let ieee_exponent: u32 = 0; // ieee_exponent = 0
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case2() {
    let ieee_mantissa: u64 = 0b1; // non-zero sequence
    let ieee_exponent: u32 = 0; // ieee_exponent = 0
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case3() {
    let ieee_mantissa: u64 = 0; // leading zero
    let ieee_exponent: u32 = 1; // valid exponent for e2 >= 0
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case4() {
    let ieee_mantissa: u64 = 0; // leading zero
    let ieee_exponent: u32 = 2047; // largest exponent, valid e2
    let result = d2d(ieee_mantissa, ieee_exponent);
}

