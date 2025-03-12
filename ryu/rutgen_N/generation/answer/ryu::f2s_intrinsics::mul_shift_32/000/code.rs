// Answer 0

#[test]
fn test_mul_shift_32() {
    // Test case: normal multiplication and shift
    let m: u32 = 10;
    let factor: u64 = 40;
    let shift: i32 = 34;
    let result = mul_shift_32(m, factor, shift);
    assert_eq!(result, 1); // Expected output: (10 * 40) >> 2

    // Test case: edge value of shift just above 32
    let m: u32 = 1;
    let factor: u64 = 1;
    let shift: i32 = 33;
    let result = mul_shift_32(m, factor, shift);
    assert_eq!(result, 0); // Expected output: (1 * 1) >> 32

    // Test case: large factor and m
    let m: u32 = u32::MAX;
    let factor: u64 = u64::MAX;
    let shift: i32 = 64;
    let result = mul_shift_32(m, factor, shift);
    assert_eq!(result, u32::MAX); // Expected output: (u32::MAX * u64::MAX) >> 32

    // Test case: another edge value
    let m: u32 = 0;
    let factor: u64 = 100;
    let shift: i32 = 36;
    let result = mul_shift_32(m, factor, shift);
    assert_eq!(result, 0); // Expected output: (0 * 100) >> 4
}

