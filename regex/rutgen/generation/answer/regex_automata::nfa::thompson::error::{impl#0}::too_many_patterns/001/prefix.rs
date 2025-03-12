// Answer 0

#[test]
fn test_too_many_patterns_zero() {
    let given = 0;
    let result = BuildError::too_many_patterns(given);
}

#[test]
fn test_too_many_patterns_limit() {
    let given = PatternID::LIMIT;
    let result = BuildError::too_many_patterns(given);
}

#[test]
fn test_too_many_patterns_above_limit() {
    let given = PatternID::LIMIT + 1;
    let result = BuildError::too_many_patterns(given);
}

