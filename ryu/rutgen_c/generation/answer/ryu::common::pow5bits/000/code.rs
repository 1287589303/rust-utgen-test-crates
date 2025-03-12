// Answer 0

#[test]
fn test_pow5bits_lower_bound() {
    assert_eq!(pow5bits(0), 1);
}

#[test]
fn test_pow5bits_middle_value() {
    assert_eq!(pow5bits(1000), 483157);
}

#[test]
fn test_pow5bits_upper_bound() {
    assert_eq!(pow5bits(3528), 787102094);
}

#[should_panic]
fn test_pow5bits_below_lower_bound() {
    pow5bits(-1);
}

#[should_panic]
fn test_pow5bits_above_upper_bound() {
    pow5bits(3529);
}

