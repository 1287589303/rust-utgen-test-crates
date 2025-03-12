// Answer 0

#[test]
fn test_mul_shift_32_case1() {
    let m: u32 = 0;
    let factor: u64 = 1;
    let shift: i32 = 33;
    mul_shift_32(m, factor, shift);
}

#[test]
fn test_mul_shift_32_case2() {
    let m: u32 = 4294967295;
    let factor: u64 = 1;
    let shift: i32 = 33;
    mul_shift_32(m, factor, shift);
}

#[test]
fn test_mul_shift_32_case3() {
    let m: u32 = 1;
    let factor: u64 = 4294967295;
    let shift: i32 = 33;
    mul_shift_32(m, factor, shift);
}

#[test]
fn test_mul_shift_32_case4() {
    let m: u32 = 2147483647;
    let factor: u64 = 18446744073709551615;
    let shift: i32 = 63;
    mul_shift_32(m, factor, shift);
}

#[test]
fn test_mul_shift_32_case5() {
    let m: u32 = 4294967295;
    let factor: u64 = 4294967295;
    let shift: i32 = 63;
    mul_shift_32(m, factor, shift);
}

