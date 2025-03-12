// Answer 0

#[test]
fn test_mul_shift_64_case1() {
    let m: u64 = 2;
    let mul: (u64, u64) = (3, 5);
    let j: u32 = 66;
    let result = mul_shift_64(m, &mul, j);
    assert_eq!(result, 0); //  (((2 * 3) >> 64) + (2 * 5)) >> (66 - 64)
}

#[test]
fn test_mul_shift_64_case2() {
    let m: u64 = 1;
    let mul: (u64, u64) = (0, 1);
    let j: u32 = 64;
    let result = mul_shift_64(m, &mul, j);
    assert_eq!(result, 0); //  (((1 * 0) >> 64) + (1 * 1)) >> (64 - 64)
}

#[test]
fn test_mul_shift_64_case3() {
    let m: u64 = 10;
    let mul: (u64, u64) = (20, 30);
    let j: u32 = 75;
    let result = mul_shift_64(m, &mul, j);
    assert_eq!(result, 0); //  (((10 * 20) >> 64) + (10 * 30)) >> (75 - 64)
}

#[test]
fn test_mul_shift_64_case4() {
    let m: u64 = 0;
    let mul: (u64, u64) = (100, 200);
    let j: u32 = 64;
    let result = mul_shift_64(m, &mul, j);
    assert_eq!(result, 0); //  (((0 * 100) >> 64) + (0 * 200)) >> (64 - 64)
}

#[test]
fn test_mul_shift_64_case5() {
    let m: u64 = 5;
    let mul: (u64, u64) = (12, 8);
    let j: u32 = 65;
    let result = mul_shift_64(m, &mul, j);
    assert_eq!(result, 0); //  (((5 * 12) >> 64) + (5 * 8)) >> (65 - 64)
}

