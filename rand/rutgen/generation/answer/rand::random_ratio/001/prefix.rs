// Answer 0

#[test]
fn test_random_ratio_numerator_zero() {
    let _result = rand::random_ratio(0, 1);
}

#[test]
fn test_random_ratio_numerator_equals_denominator() {
    let _result = rand::random_ratio(5, 5);
}

#[test]
fn test_random_ratio_numerator_less_than_denominator() {
    let _result = rand::random_ratio(3, 7);
}

#[test]
#[should_panic]
fn test_random_ratio_denominator_zero() {
    let _result = rand::random_ratio(1, 0);
}

#[test]
#[should_panic]
fn test_random_ratio_numerator_greater_than_denominator() {
    let _result = rand::random_ratio(6, 5);
}

