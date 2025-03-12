// Answer 0

#[test]
fn test_log10_pow5_lower_bound() {
    let e: i32 = 0;
    let result = log10_pow5(e);
    let expected = (e as u32 * 732923) >> 20;
    assert_eq!(result, expected);
}

#[test]
fn test_log10_pow5_upper_bound() {
    let e: i32 = 2620;
    let result = log10_pow5(e);
    let expected = (e as u32 * 732923) >> 20;
    assert_eq!(result, expected);
}

