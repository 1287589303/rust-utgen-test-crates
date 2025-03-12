// Answer 0

#[test]
fn test_log2_pow5_zero() {
    let result = log2_pow5(0);
}

#[test]
fn test_log2_pow5_one() {
    let result = log2_pow5(1);
}

#[test]
fn test_log2_pow5_boundary() {
    let result = log2_pow5(3528);
}

#[should_panic]
#[test]
fn test_log2_pow5_above_upper_bound() {
    let result = log2_pow5(3529);
}

#[should_panic]
#[test]
fn test_log2_pow5_below_lower_bound() {
    let result = log2_pow5(-1);
}

#[should_panic]
#[test]
fn test_log2_pow5_large_value() {
    let result = log2_pow5(4000);
}

