// Answer 0

#[test]
fn test_log10_pow5_with_e_zero() {
    let result = log10_pow5(0);
    assert_eq!(result, 0);
}

#[should_panic]
#[test]
fn test_log10_pow5_with_e_above_bound() {
    log10_pow5(2621);
}

