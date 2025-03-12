// Answer 0

#[test]
fn test_d2d_case_1() {
    let ieee_mantissa: u64 = 1;
    let ieee_exponent: u32 = 1024;
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case_2() {
    let ieee_mantissa: u64 = (1 << 52) - 1;
    let ieee_exponent: u32 = 2047;
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case_3() {
    let ieee_mantissa: u64 = 12345678901234;
    let ieee_exponent: u32 = 1500;
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case_4() {
    let ieee_mantissa: u64 = 98765432101234;
    let ieee_exponent: u32 = 1700;
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case_5() {
    let ieee_mantissa: u64 = 50000000000000;
    let ieee_exponent: u32 = 1200;
    let result = d2d(ieee_mantissa, ieee_exponent);
}

