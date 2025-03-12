// Answer 0

#[test]
fn test_ceil_log2_pow5_positive() {
    assert_eq!(ryu::ceil_log2_pow5(1), 2);
    assert_eq!(ryu::ceil_log2_pow5(31), 6);
    assert_eq!(ryu::ceil_log2_pow5(32), 6);
}

#[test]
fn test_ceil_log2_pow5_zero() {
    assert_eq!(ryu::ceil_log2_pow5(0), 1);
}

#[test]
#[should_panic]
fn test_ceil_log2_pow5_negative() {
    ryu::ceil_log2_pow5(-1);
}

