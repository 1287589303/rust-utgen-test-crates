// Answer 0

#[test]
fn test_log10_pow2_e_is_zero() {
    let result = log10_pow2(0);
    assert_eq!(result, 0);
}

#[should_panic]
fn test_log10_pow2_e_greater_than_1650() {
    log10_pow2(1651);
}

