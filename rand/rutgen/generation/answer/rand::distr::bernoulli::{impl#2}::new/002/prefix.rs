// Answer 0

#[test]
fn test_bernoulli_new_probability_one() {
    let result = Bernoulli::new(1.0);
    let expected = Ok(Bernoulli { p_int: ALWAYS_TRUE });
    let _ = (result, expected);
}

#[test]
fn test_bernoulli_new_probability_out_of_bounds_too_high() {
    let result = Bernoulli::new(1.1);
    let expected = Err(BernoulliError::InvalidProbability);
    let _ = (result, expected);
}

#[test]
fn test_bernoulli_new_probability_out_of_bounds_negative() {
    let result = Bernoulli::new(-0.1);
    let expected = Err(BernoulliError::InvalidProbability);
    let _ = (result, expected);
}

