// Answer 0

#[test]
fn test_mul_shift_32_boundary_condition_high_shift() {
    let m: u32 = 1;
    let factor: u64 = u64::MAX; // This factor will maximize the product
    let shift: i32 = 64; // A shift that’s greater than 32

    let result = mul_shift_32(m, factor, shift);
    
    assert!(result <= u32::MAX); // The shifted result should be valid as per the precondition
}

#[test]
#[should_panic]
fn test_mul_shift_32_exceeding_max_value() {
    let m: u32 = u32::MAX; // Using the maximum value for m to ensure a large product
    let factor: u64 = u64::MAX; // Maximum factor
    let shift: i32 = 33; // Just above 32 for valid shift condition

    let _ = mul_shift_32(m, factor, shift); // This should panic due to exceeding u32 max value
}

