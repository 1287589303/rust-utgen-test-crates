// Answer 0

#[test]
fn test_d2d_case1() {
    let ieee_mantissa: u64 = 0x0000000000000001;
    let ieee_exponent: u32 = 0;
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case2() {
    let ieee_mantissa: u64 = 0x0000000000000010;
    let ieee_exponent: u32 = 0;
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case3() {
    let ieee_mantissa: u64 = 0x0000000000000100;
    let ieee_exponent: u32 = 0;
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case4() {
    let ieee_mantissa: u64 = 0x0000000000001000;
    let ieee_exponent: u32 = 0;
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case5() {
    let ieee_mantissa: u64 = 0x0000000000010000;
    let ieee_exponent: u32 = 0;
    let result = d2d(ieee_mantissa, ieee_exponent);
}

