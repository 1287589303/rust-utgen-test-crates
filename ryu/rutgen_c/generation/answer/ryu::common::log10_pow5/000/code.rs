// Answer 0

#[test]
fn test_log10_pow5() {
    assert_eq!(log10_pow5(0), 0);
    assert_eq!(log10_pow5(1), 34);
    assert_eq!(log10_pow5(2620), 831168);
}

#[test]
#[should_panic]
fn test_log10_pow5_below_lower_bound() {
    log10_pow5(-1);
}

#[test]
#[should_panic]
fn test_log10_pow5_above_upper_bound() {
    log10_pow5(2621);
}

#[test]
fn test_log10_pow5_boundary() {
    assert_eq!(log10_pow5(10), 36);
    assert_eq!(log10_pow5(100), 72);
    assert_eq!(log10_pow5(2000), 696);
}

