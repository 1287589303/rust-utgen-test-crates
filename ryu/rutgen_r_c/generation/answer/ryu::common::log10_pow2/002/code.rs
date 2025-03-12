// Answer 0

#[test]
fn test_log10_pow2_bound_zero() {
    let e: i32 = 0;
    assert_eq!(log10_pow2(e), 0);
}

#[should_panic]
fn test_log10_pow2_above_upper_bound() {
    let e: i32 = 1651;
    log10_pow2(e);
}

