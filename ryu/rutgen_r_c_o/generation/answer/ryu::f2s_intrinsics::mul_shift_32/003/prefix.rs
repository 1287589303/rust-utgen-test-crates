// Answer 0

#[test]
#[should_panic]
fn test_mul_shift_32_shift_eq_32() {
    let m: u32 = 0; // test lower bound for m
    let factor: u64 = 0; // test lower bound for factor
    let shift: i32 = 32; // shift == 32
    let _result = mul_shift_32(m, factor, shift);
}

#[test]
#[should_panic]
fn test_mul_shift_32_shift_eq_32_m_max() {
    let m: u32 = 4294967295; // test upper bound for m
    let factor: u64 = 0; // test lower bound for factor
    let shift: i32 = 32; // shift == 32
    let _result = mul_shift_32(m, factor, shift);
}

#[test]
#[should_panic]
fn test_mul_shift_32_shift_eq_32_factor_max() {
    let m: u32 = 0; // test lower bound for m
    let factor: u64 = 18446744073709551615; // test upper bound for factor
    let shift: i32 = 32; // shift == 32
    let _result = mul_shift_32(m, factor, shift);
}

#[test]
#[should_panic]
fn test_mul_shift_32_shift_eq_32_max_values() {
    let m: u32 = 4294967295; // test upper bound for m
    let factor: u64 = 18446744073709551615; // test upper bound for factor
    let shift: i32 = 32; // shift == 32
    let _result = mul_shift_32(m, factor, shift);
}

