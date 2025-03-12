// Answer 0

#[test]
fn test_mul_shift_64_case1() {
    let m: u64 = 0;
    let mul: (u64, u64) = (0, 0);
    let j: u32 = 64;
    let _ = mul_shift_64(m, &mul, j);
}

#[test]
fn test_mul_shift_64_case2() {
    let m: u64 = 1;
    let mul: (u64, u64) = (1, 1);
    let j: u32 = 64;
    let _ = mul_shift_64(m, &mul, j);
}

#[test]
fn test_mul_shift_64_case3() {
    let m: u64 = u64::MAX;
    let mul: (u64, u64) = (u64::MAX, u64::MAX);
    let j: u32 = 64;
    let _ = mul_shift_64(m, &mul, j);
}

#[test]
fn test_mul_shift_64_case4() {
    let m: u64 = 0;
    let mul: (u64, u64) = (u64::MAX, u64::MAX);
    let j: u32 = 128;
    let _ = mul_shift_64(m, &mul, j);
}

#[test]
fn test_mul_shift_64_case5() {
    let m: u64 = 12345;
    let mul: (u64, u64) = (67890, 13579);
    let j: u32 = 64;
    let _ = mul_shift_64(m, &mul, j);
}

#[test]
fn test_mul_shift_64_case6() {
    let m: u64 = u64::MAX;
    let mul: (u64, u64) = (0, 0);
    let j: u32 = 128;
    let _ = mul_shift_64(m, &mul, j);
}

#[test]
fn test_mul_shift_64_case7() {
    let m: u64 = u64::MAX - 1;
    let mul: (u64, u64) = (1, 0);
    let j: u32 = 64;
    let _ = mul_shift_64(m, &mul, j);
}

#[test]
fn test_mul_shift_64_case8() {
    let m: u64 = 0;
    let mul: (u64, u64) = (1, 1);
    let j: u32 = 128;
    let _ = mul_shift_64(m, &mul, j);
}

