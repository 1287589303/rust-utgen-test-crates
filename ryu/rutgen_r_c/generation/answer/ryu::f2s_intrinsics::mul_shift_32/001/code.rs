// Answer 0

#[test]
fn test_mul_shift_32_shifted_sum_equals_max_value() {
    let m: u32 = 1;
    let factor: u64 = 0xFFFFFFFFFFFFFFFF; // maximum factor (as u64)
    let shift: i32 = 64; // shift > 32

    let result = mul_shift_32(m, factor, shift);
    assert_eq!(result, u32::max_value()); // shifted_sum should equal u32::max_value()
}

#[test]
fn test_mul_shift_32_boundary_condition() {
    let m: u32 = 0xFFFFFFFF; // maximum value for u32
    let factor: u64 = 2; // small factor
    let shift: i32 = 33; // shift just above 32

    let result = mul_shift_32(m, factor, shift);
    assert_eq!(result, 0xFFFFFFFF - 1); // shifted_sum should be valid and within range
}

