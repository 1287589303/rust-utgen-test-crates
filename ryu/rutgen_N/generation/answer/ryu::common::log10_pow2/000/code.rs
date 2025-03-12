// Answer 0

#[test]
fn test_log10_pow2_lower_bound() {
    assert_eq!(log10_pow2(0), 0);
}

#[test]
fn test_log10_pow2_mid_range() {
    assert_eq!(log10_pow2(1650), 78913);
}

#[test]
fn test_log10_pow2_upper_bound() {
    assert_eq!(log10_pow2(1650), 78913);
}

#[should_panic(expected = "assertion failed")]
fn test_log10_pow2_negative_input() {
    log10_pow2(-1);
}

#[should_panic(expected = "assertion failed")]
fn test_log10_pow2_out_of_bounds() {
    log10_pow2(1651);
}

