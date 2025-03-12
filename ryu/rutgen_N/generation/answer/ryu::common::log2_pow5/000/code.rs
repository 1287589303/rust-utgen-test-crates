// Answer 0

#[test]
fn test_log2_pow5_lower_bound() {
    assert_eq!(ryu::log2_pow5(0), 0);
}

#[test]
fn test_log2_pow5_middle_values() {
    assert_eq!(ryu::log2_pow5(1000), 476);
    assert_eq!(ryu::log2_pow5(2000), 952);
    assert_eq!(ryu::log2_pow5(3000), 1429);
}

#[test]
fn test_log2_pow5_upper_bound() {
    assert_eq!(ryu::log2_pow5(3528), 1668);
}

#[should_panic]
fn test_log2_pow5_negative() {
    let _ = ryu::log2_pow5(-1);
}

#[should_panic]
fn test_log2_pow5_above_upper_bound() {
    let _ = ryu::log2_pow5(3529);
}

