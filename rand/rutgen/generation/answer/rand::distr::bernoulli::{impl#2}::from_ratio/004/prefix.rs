// Answer 0

#[test]
fn test_from_ratio_numerator_zero_denominator_non_zero() {
    let result = Bernoulli::from_ratio(0, 1);
}

#[test]
fn test_from_ratio_numerator_one_denominator_non_zero() {
    let result = Bernoulli::from_ratio(1, 1);
}

#[test]
#[should_panic]
fn test_from_ratio_numerator_greater_denominator() {
    let result = Bernoulli::from_ratio(2, 1);
}

#[test]
#[should_panic]
fn test_from_ratio_denominator_zero() {
    let result = Bernoulli::from_ratio(1, 0);
}

