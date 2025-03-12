// Answer 0

#[test]
fn test_pattern_epsilons_is_empty_with_zero_value() {
    let pattern_epsilons = PatternEpsilons(0);
    let _ = pattern_epsilons.is_empty();
}

#[test]
fn test_pattern_epsilons_is_empty_with_pattern_id_zero() {
    let pattern_epsilons = PatternEpsilons(PatternEpsilons::PATTERN_ID_LIMIT);
    let _ = pattern_epsilons.is_empty();
}

#[test]
fn test_pattern_epsilons_is_empty_with_epsilons_zero() {
    let epsilons = Epsilons::empty();
    let pattern_epsilons = PatternEpsilons(0 | epsilons.0);
    let _ = pattern_epsilons.is_empty();
}

