// Answer 0

#[test]
fn test_ceil_log2_pow5_lower_bound() {
    let result = ceil_log2_pow5(0);
    assert_eq!(result, 1);
}

#[test]
fn test_ceil_log2_pow5_middle_value() {
    let result = ceil_log2_pow5(1000);
    assert_eq!(result, log2_pow5(1000) + 1);
}

#[test]
fn test_ceil_log2_pow5_upper_bound() {
    let result = ceil_log2_pow5(3528);
    assert_eq!(result, log2_pow5(3528) + 1);
}

#[test]
#[should_panic]
fn test_ceil_log2_pow5_below_lower_bound() {
    ceil_log2_pow5(-1);
}

#[test]
#[should_panic]
fn test_ceil_log2_pow5_above_upper_bound() {
    ceil_log2_pow5(3529);
}

