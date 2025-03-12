// Answer 0

#[test]
fn test_log2_pow5_lower_boundary() {
    assert_eq!(log2_pow5(0), 0);
}

#[test]
fn test_log2_pow5_middle_value() {
    assert_eq!(log2_pow5(1764), 1000000); // example expected value
}

#[test]
fn test_log2_pow5_upper_boundary() {
    assert_eq!(log2_pow5(3528), 2000000); // example expected value
}

#[should_panic]
fn test_log2_pow5_negative_value() {
    log2_pow5(-1);
}

#[should_panic]
fn test_log2_pow5_exceeding_upper_boundary() {
    log2_pow5(3529);
}

