// Answer 0

#[test]
fn test_d2d_case1() {
    let ieee_mantissa: u64 = 0;
    let ieee_exponent: u32 = 2047;
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case2() {
    let ieee_mantissa: u64 = 1;
    let ieee_exponent: u32 = 2047;
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case3() {
    let ieee_mantissa: u64 = 0;
    let ieee_exponent: u32 = 2046; 
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case4() {
    let ieee_mantissa: u64 = 0;
    let ieee_exponent: u32 = 2048; 
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case5() {
    let ieee_mantissa: u64 = 0;
    let ieee_exponent: u32 = 2047; 
    let result = d2d(ieee_mantissa, ieee_exponent);
}

