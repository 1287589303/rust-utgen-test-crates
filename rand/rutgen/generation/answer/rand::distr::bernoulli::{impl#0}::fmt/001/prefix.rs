// Answer 0

#[test]
fn test_display_invalid_probability_negative() {
    let error = BernoulliError::InvalidProbability;
    let mut output = String::new();
    let _ = write!(&mut output, "{}", error);
}

#[test]
fn test_display_invalid_probability_zero() {
    let error = BernoulliError::InvalidProbability; 
    let mut output = String::new();
    let _ = write!(&mut output, "{}", error);
}

#[test]
fn test_display_invalid_probability_half() {
    let error = BernoulliError::InvalidProbability;
    let mut output = String::new();
    let _ = write!(&mut output, "{}", error);
}

#[test]
fn test_display_invalid_probability_one() {
    let error = BernoulliError::InvalidProbability;
    let mut output = String::new();
    let _ = write!(&mut output, "{}", error);
}

#[test]
fn test_display_invalid_probability_above_one() {
    let error = BernoulliError::InvalidProbability;
    let mut output = String::new();
    let _ = write!(&mut output, "{}", error);
}

