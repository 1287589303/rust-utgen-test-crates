// Answer 0

#[test]
fn test_d2d_ieee_exponent_zero() {
    let ieee_mantissa: u64 = 123; // non-zero mantissa
    let ieee_exponent: u32 = 0; // ieee_exponent is zero
    
    let result = d2d(ieee_mantissa, ieee_exponent);
    
    assert_eq!(result.mantissa, ieee_mantissa);
}

#[test]
fn test_d2d_e2_non_negative() {
    let ieee_mantissa: u64 = 1; // valid non-zero mantissa
    let ieee_exponent: u32 = 2047; // causes e2 to be < 0

    let result = d2d(ieee_mantissa, ieee_exponent);
    
    assert!(result.exponent < 0); // e2 < 0 should hold
}

#[test]
fn test_d2d_double_pow5_split_bounds() {
    struct DoublePow5Split<T> {
        data: Vec<T>,
    }

    let ieee_mantissa: u64 = 1; // valid non-zero mantissa
    let ieee_exponent: u32 = 1023; // results in i == DOUBLE_POW5_SPLIT.len() as i32

    let double_pow5_split = DoublePow5Split { 
        data: vec![0; 10] // Assume it has 10 elements for this example
    };

    let result = d2d(ieee_mantissa, ieee_exponent);

    assert!(result.exponent >= 0);
}

