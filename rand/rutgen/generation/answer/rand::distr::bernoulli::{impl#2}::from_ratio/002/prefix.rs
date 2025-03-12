// Answer 0

#[test]
fn test_from_ratio_numerator_greater_than_denominator() {
    let numerator = 101;
    let denominator = 100;
    let result = Bernoulli::from_ratio(numerator, denominator);
}

#[test]
fn test_from_ratio_denominator_zero() {
    let numerator = 100;
    let denominator = 0;
    let result = Bernoulli::from_ratio(numerator, denominator);
}

#[test]
fn test_from_ratio_numerator_equals_denominator() {
    let numerator = 50;
    let denominator = 50;
    let result = Bernoulli::from_ratio(numerator, denominator);
}

#[test]
fn test_from_ratio_numerator_zero_denominator_valid() {
    let numerator = 0;
    let denominator = 1;
    let result = Bernoulli::from_ratio(numerator, denominator);
}

