// Answer 0

#[test]
fn test_log2_pow5_with_zero() {
    let result = ryu::log2_pow5(0);
    assert_eq!(result, 0);
}

#[test]
#[should_panic]
fn test_log2_pow5_exceeds_upper_bound() {
    ryu::log2_pow5(3529);
}

