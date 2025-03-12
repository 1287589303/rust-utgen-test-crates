// Answer 0

#[test]
fn test_mul_shift_64_basic() {
    let m: u64 = 2;
    let mul: (u64, u64) = (3, 4);
    let j: u32 = 64;
    let result = mul_shift_64(m, &mul, j);
    assert_eq!(result, 1);
}

#[test]
fn test_mul_shift_64_large_mul() {
    let m: u64 = 1 << 32;  // 4294967296
    let mul: (u64, u64) = (0xFFFFFFFFFFFFFFFF, 0x1);
    let j: u32 = 128;
    let result = mul_shift_64(m, &mul, j);
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF);
}

#[test]
fn test_mul_shift_64_boundary_condition() {
    let m: u64 = 1;
    let mul: (u64, u64) = (1, 1);
    let j: u32 = 65;
    let result = mul_shift_64(m, &mul, j);
    assert_eq!(result, 0);
}

#[test]
fn test_mul_shift_64_zero_multiplier() {
    let m: u64 = 10;
    let mul: (u64, u64) = (0, 0);
    let j: u32 = 64;
    let result = mul_shift_64(m, &mul, j);
    assert_eq!(result, 0);
}

#[test]
fn test_mul_shift_64_high_value() {
    let m: u64 = u64::MAX; // 18446744073709551615
    let mul: (u64, u64) = (1, 1);
    let j: u32 = 128;
    let result = mul_shift_64(m, &mul, j);
    assert_eq!(result, 2);
}

