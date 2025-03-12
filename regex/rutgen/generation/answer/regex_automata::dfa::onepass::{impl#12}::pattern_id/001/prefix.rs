// Answer 0

#[test]
fn test_pattern_id_limit() {
    let pattern_epsilons = PatternEpsilons(0x00000000_003FFFFF);
    let result = pattern_epsilons.pattern_id();
}

#[test]
fn test_pattern_id_non_limit() {
    let pattern_epsilons = PatternEpsilons(0x00000000_003FFFFE);
    let result = pattern_epsilons.pattern_id();
}

