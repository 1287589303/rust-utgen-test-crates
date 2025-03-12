// Answer 0

#[test]
fn test_from_ratio_numerator_equals_denominator_valid() {
    let result = Bernoulli::from_ratio(1, 1);
}

#[test]
#[should_panic]
fn test_from_ratio_denominator_zero() {
    let result = Bernoulli::from_ratio(1, 0);
}

#[test]
fn test_from_ratio_numerator_zero() {
    let result = Bernoulli::from_ratio(0, 1);
}

#[test]
fn test_from_ratio_large_equal_numerator_denominator() {
    let result = Bernoulli::from_ratio(2, 2);
}

#[test]
fn test_from_ratio_small_equal_numerator_denominator() {
    let result = Bernoulli::from_ratio(3, 3);
}

#[test]
fn test_from_ratio_numerator_greater_than_denominator() {
    let result = Bernoulli::from_ratio(3, 2);
}

#[test]
#[should_panic]
fn test_from_ratio_zero_zero() {
    let result = Bernoulli::from_ratio(0, 0);
}

