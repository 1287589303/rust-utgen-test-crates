// Answer 0

#[test]
fn test_log10_pow5_valid_range() {
    assert_eq!(log10_pow5(0), 0);
    assert_eq!(log10_pow5(1), 34);
    assert_eq!(log10_pow5(2620), 268435);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_log10_pow5_negative_input() {
    log10_pow5(-1);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_log10_pow5_exceeding_upper_bound() {
    log10_pow5(2621);
}

