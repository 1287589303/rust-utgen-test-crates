// Answer 0

#[test]
fn test_d2d_ieee_exponent_zero() {
    let ieee_mantissa = 1;
    let ieee_exponent = 0;
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.exponent < 0);
    assert!(result.mantissa > 0);
}

#[test]
fn test_d2d_e2_negative() {
    let ieee_mantissa = 1;
    let ieee_exponent = 2047; // max exponent in IEEE 754
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.exponent < 0);
}

#[test]
fn test_d2d_double_pow5_split_length() {
    let ieee_mantissa = u64::MAX; // ensure it fits for testing
    let ieee_exponent = 2046; // just below the max exponent
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(DOUBLE_POW5_SPLIT.len() > 0);
}

#[test]
fn test_d2d_q_less_than_or_equal_one() {
    let ieee_mantissa = 8; // producing q <= 1
    let ieee_exponent = 1023; // normal exponent
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.exponent, 0);
}

#[test]
fn test_d2d_accept_bounds_false() {
    let ieee_mantissa = 2;
    let ieee_exponent = 1015; // Ensures accept_bounds is false
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.mantissa > 0);
}

#[test]
fn test_d2d_vp_div10_leq_vm_div10() {
    let ieee_mantissa = 500; // 500 will cause vp_div10 to be 50, it works
    let ieee_exponent = 1022; // a reasonable exponent
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.mantissa % 10 == 0); 
}

#[test]
fn test_d2d_vm_trailing_zeros_false() {
    let ieee_mantissa = 3;
    let ieee_exponent = 1023; // normal range
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(!result.mantissa % 10 == 0); 
}

#[test]
fn test_d2d_vr_not_equal_vm() {
    let ieee_mantissa = 7; 
    let ieee_exponent = 1023; // normal exponent value
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.vr != result.vm);
}

