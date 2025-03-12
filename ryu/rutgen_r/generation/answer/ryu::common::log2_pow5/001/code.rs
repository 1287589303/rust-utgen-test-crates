// Answer 0

#[test]
fn test_log2_pow5_zero() {
    let e: i32 = 0;
    let expected: i32 = ((e as u32 * 1217359) >> 19) as i32;
    assert_eq!(ryu::log2_pow5(e), expected);
}

#[test]
fn test_log2_pow5_max() {
    let e: i32 = 3528;
    let expected: i32 = ((e as u32 * 1217359) >> 19) as i32;
    assert_eq!(ryu::log2_pow5(e), expected);
}

