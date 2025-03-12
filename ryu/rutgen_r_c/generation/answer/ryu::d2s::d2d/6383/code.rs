// Answer 0

#[test]
fn test_d2d_exponent_zero_non_zero_mantissa() {
    let ieee_mantissa: u64 = 1;
    let ieee_exponent: u32 = 0;
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_ne!(result.exponent, 0);
    assert_ne!(result.mantissa, 0);
}

#[test]
fn test_d2d_non_zero_exponent_positive_mantissa() {
    let ieee_mantissa: u64 = 1;
    let ieee_exponent: u32 = 2048; // Example positive exponent
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.exponent >= 0);
    assert!(result.mantissa > 0);
}

#[test]
fn test_d2d_invalid_double_pow5_split_index() {
    let ieee_mantissa: u64 = 1;
    let ieee_exponent: u32 = 2046; // Adjust so that it triggers the condition
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.mantissa > 0);
}

#[test]
fn test_d2d_q_is_63() {
    let ieee_mantissa: u64 = 1 << 52; // Example value that would yield q == 63
    let ieee_exponent: u32 = 2048; // Adjust accordingly
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.exponent > 0);
    assert!(result.mantissa > 0);
}

#[test]
fn test_d2d_vm_is_not_trailing_zeros() {
    let ieee_mantissa: u64 = 3; // Example value
    let ieee_exponent: u32 = 2047;
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(!result.mantissa.is_power_of_two());
}

#[test]
fn test_d2d_vr_is_trailing_zeros() {
    let ieee_mantissa: u64 = 1; // Value that should lead to trailing zeros
    let ieee_exponent: u32 = 2047;
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.mantissa.trailing_zeros() > 0);
}

#[test]
fn test_d2d_vp_div10_greater_than_vm_div10() {
    let ieee_mantissa: u64 = 8; // Example where vp_div10 > vm_div10
    let ieee_exponent: u32 = 2047;
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.mantissa > 1); // Adjust condition as appropriate
}

#[test]
fn test_d2d_vp_div10_equal_vm_div10() {
    let ieee_mantissa: u64 = 100; // Configure to ensure equal division
    let ieee_exponent: u32 = 2047;
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.mantissa % 10, 0);
}

#[test]
fn test_d2d_vm_is_trailing_zeros() {
    let ieee_mantissa: u64 = 0; // Example value that results in vm being trailing zeros
    let ieee_exponent: u32 = 2047;
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.mantissa.is_power_of_two());
}

#[test]
fn test_d2d_vm_mod10_not_zero() {
    let ieee_mantissa: u64 = 40; // Example to ensure this condition
    let ieee_exponent: u32 = 2047;
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_ne!(result.mantissa % 10, 0);
}

#[test]
fn test_d2d_vr_is_not_trailing_zeros() {
    let ieee_mantissa: u64 = 3; // Example to check for this assertion
    let ieee_exponent: u32 = 2047;
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(!result.mantissa.trailing_zeros().is_zero());
}

#[test]
fn test_d2d_vr_not_equal_vm() {
    let ieee_mantissa: u64 = 1; // Case where vr != vm
    let ieee_exponent: u32 = 2048; // Adjust as needed for this case
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_ne!(result.mantissa, result.mantissa);
}

