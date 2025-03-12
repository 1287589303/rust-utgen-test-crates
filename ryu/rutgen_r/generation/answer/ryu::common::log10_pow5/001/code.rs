// Answer 0

#[test]
fn test_log10_pow5_zero() {
    let e = 0;
    let expected = (e as u32 * 732923) >> 20;
    assert_eq!(ryu::log10_pow5(e), expected);
}

#[test]
fn test_log10_pow5_boundary() {
    let e = 2620;
    let expected = (e as u32 * 732923) >> 20;
    assert_eq!(ryu::log10_pow5(e), expected);
}

