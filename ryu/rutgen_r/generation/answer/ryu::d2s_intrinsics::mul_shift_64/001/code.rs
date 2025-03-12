// Answer 0

#[test]
fn test_mul_shift_64_case1() {
    let m: u64 = 1;
    let mul: (u64, u64) = (2, 3);
    let j: u32 = 64;
    let result = mul_shift_64(m, &mul, j);
    assert_eq!(result, 0);
}

#[test]
fn test_mul_shift_64_case2() {
    let m: u64 = 2;
    let mul: (u64, u64) = (2, 3);
    let j: u32 = 65;
    let result = mul_shift_64(m, &mul, j);
    assert_eq!(result, 1);
}

#[test]
fn test_mul_shift_64_case3() {
    let m: u64 = 4;
    let mul: (u64, u64) = (2, 5);
    let j: u32 = 66;
    let result = mul_shift_64(m, &mul, j);
    assert_eq!(result, 1);
}

#[test]
fn test_mul_shift_64_case4() {
    let m: u64 = 0;
    let mul: (u64, u64) = (2, 3);
    let j: u32 = 64;
    let result = mul_shift_64(m, &mul, j);
    assert_eq!(result, 0);
}

#[test]
fn test_mul_shift_64_case5() {
    let m: u64 = 10;
    let mul: (u64, u64) = (6, 0);
    let j: u32 = 64;
    let result = mul_shift_64(m, &mul, j);
    assert_eq!(result, 0);
}

