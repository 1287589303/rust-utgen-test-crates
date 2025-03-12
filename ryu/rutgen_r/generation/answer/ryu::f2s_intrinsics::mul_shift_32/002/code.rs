// Answer 0

#[test]
fn test_mul_shift_32_overflow() {
    let m: u32 = 1;
    let factor: u64 = u32::max_value() as u64 + 1; // This will ensure the product exceeds u32::max_value()
    let shift: i32 = 33; // shift is greater than 32

    let result = mul_shift_32(m, factor, shift);
    assert!(result > u32::max_value()); // Ensure we check for overflow
}

#[test]
fn test_mul_shift_32_edge_case() {
    let m: u32 = u32::max_value(); // Max value for m
    let factor: u64 = 2; // A small factor that when multiplied will cause sum to exceed u32::max_value()
    let shift: i32 = 36; // shift is greater than 32

    let result = mul_shift_32(m, factor, shift);
    assert!(result > u32::max_value()); // Ensure we check for overflow
}

