// Answer 0

#[test]
fn test_log10_pow2_zero() {
    let e: i32 = 0;
    let expected: u32 = (e as u32 * 78913) >> 18;
    assert_eq!(ryu::log10_pow2(e), expected);
}

#[test]
fn test_log10_pow2_bound() {
    let e: i32 = 1650;
    let expected: u32 = (e as u32 * 78913) >> 18;
    assert_eq!(ryu::log10_pow2(e), expected);
}

