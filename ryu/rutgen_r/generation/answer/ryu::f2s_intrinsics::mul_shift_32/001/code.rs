// Answer 0

#[test]
fn test_mul_shift_32_large_factor() {
    let m: u32 = 1;
    let factor: u64 = u32::max_value() as u64;
    let shift: i32 = 33; // shift > 32
    let result = mul_shift_32(m, factor, shift);
    assert_eq!(result, u32::max_value()); // shifted_sum == u32::max_value()
}

#[test]
fn test_mul_shift_32_max_m_value() {
    let m: u32 = u32::max_value();
    let factor: u64 = 1;
    let shift: i32 = 34; // shift > 32
    let result = mul_shift_32(m, factor, shift);
    assert_eq!(result, 0); // shifted_sum < u32::max_value()
}

#[test]
fn test_mul_shift_32_incremental_factor() {
    let m: u32 = 10;
    let factor: u64 = 5;
    let shift: i32 = 35; // shift > 32
    let result = mul_shift_32(m, factor, shift);
    assert_eq!(result, 0); // shifted_sum < u32::max_value()
}

