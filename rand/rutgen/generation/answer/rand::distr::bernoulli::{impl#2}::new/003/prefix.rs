// Answer 0

#[test]
fn test_bernoulli_new_probability_negative() {
    let result = Bernoulli::new(-0.1);
}

#[test]
fn test_bernoulli_new_probability_greater_than_one() {
    let result = Bernoulli::new(1.1);
}

#[test]
fn test_bernoulli_new_probability_nan() {
    let result = Bernoulli::new(f64::NAN);
}

#[test]
fn test_bernoulli_new_probability_infinity() {
    let result = Bernoulli::new(f64::INFINITY);
}

#[test]
fn test_bernoulli_new_probability_negative_infinity() {
    let result = Bernoulli::new(f64::NEG_INFINITY);
}

