// Answer 0

#[test]
fn test_epsilons_zero() {
    let pattern_epsilons = PatternEpsilons(0);
    let result = pattern_epsilons.epsilons();
}

#[test]
fn test_epsilons_one() {
    let pattern_epsilons = PatternEpsilons(1);
    let result = pattern_epsilons.epsilons();
}

#[test]
fn test_epsilons_two() {
    let pattern_epsilons = PatternEpsilons(2);
    let result = pattern_epsilons.epsilons();
}

#[test]
fn test_epsilons_max_epsilons_mask() {
    let pattern_epsilons = PatternEpsilons(0x000003FF_FFFFFFFF);
    let result = pattern_epsilons.epsilons();
}

#[test]
fn test_epsilons_max_u64() {
    let pattern_epsilons = PatternEpsilons(0xFFFFFFFFFFFFFFFF);
    let result = pattern_epsilons.epsilons();
}

