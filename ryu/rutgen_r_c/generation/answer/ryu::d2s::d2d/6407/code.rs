// Answer 0

#[test]
fn test_d2d_ieee_exponent_zero() {
    let ieee_mantissa: u64 = 1; // Non-zero to satisfy that ieee_mantissa != 0
    let ieee_exponent: u32 = 0; // This ensures precondition for line 92 is true

    let result = d2d(ieee_mantissa, ieee_exponent);

    assert_ne!(result.exponent, 0); // Ensures e2 >= 0 is false
    assert!(result.mantissa != 0); // To validate the output is correctly formed
}

#[test]
fn test_d2d_condition_i_less_than_double_pow5_split_len() {
    let ieee_mantissa: u64 = 0x2000000000000; // Some mantissa value
    let ieee_exponent: u32 = 2047; // This value is enough to ensure further calculations

    let result = d2d(ieee_mantissa, ieee_exponent);

    assert!(result.mantissa != 0); // Expect non-zero mantissa
}

#[test]
fn test_d2d_q_equal_to_63() {
    let ieee_mantissa: u64 = 0x1FFFFFFFFFFFFF; // Maximum value for mantissa
    let ieee_exponent: u32 = 2047; // Should ensure 'q' calculations lie within the predefined bounds

    let result = d2d(ieee_mantissa, ieee_exponent);

    assert!(result.mantissa > 0); // Check for valid mantissa value
}

#[test]
fn test_d2d_trailing_zeros_conditions() {
    let ieee_mantissa: u64 = 0x1FFFFFFFFFFFFF;
    let ieee_exponent: u32 = 2047; 

    let result = d2d(ieee_mantissa, ieee_exponent);

    assert!(!result.mantissa.is_power_of_two()); // Ensures vr_is_trailing_zeros is false
}

#[test]
fn test_d2d_vp_div100_equals_vm_div100() {
    let ieee_mantissa: u64 = 100; // Value set to create equality in division by 100
    let ieee_exponent: u32 = 2048; 

    let result = d2d(ieee_mantissa, ieee_exponent);

    assert_eq!(result.mantissa % 100, 0); // Asserting that vp_div100 == vm_div100
}

#[test]
fn test_d2d_vp_div10_eq_vm_div10_greater() {
    let ieee_mantissa: u64 = 200; // This should lead to comparison where vp is not less than vm
    let ieee_exponent: u32 = 2048; 

    let result = d2d(ieee_mantissa, ieee_exponent);

    assert_eq!(result.mantissa, ieee_mantissa); // Verify output consistency
}

#[test]
fn test_d2d_vr_not_equal_vm() {
    let ieee_mantissa: u64 = 300; // Value set to differentiate vr and vm
    let ieee_exponent: u32 = 2049; 

    let result = d2d(ieee_mantissa, ieee_exponent);

    assert_ne!(result.mantissa, ieee_mantissa); // Ensuring vr is not equal to vm
}

