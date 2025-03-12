// Answer 0

#[test]
fn test_log10_pow2_boundary_lower() {
    let e: i32 = 0;
    let expected: u32 = (e as u32 * 78913) >> 18;
    let result = log10_pow2(e);
    assert_eq!(result, expected);
}

#[test]
fn test_log10_pow2_boundary_upper() {
    let e: i32 = 1650;
    let expected: u32 = (e as u32 * 78913) >> 18;
    let result = log10_pow2(e);
    assert_eq!(result, expected);
}

