// Answer 0

#[test]
fn test_pattern_id_unchecked_valid_min() {
    let epsilons = PatternEpsilons(0);
    let _result = epsilons.pattern_id_unchecked();
}

#[test]
fn test_pattern_id_unchecked_valid_max() {
    let epsilons = PatternEpsilons(0xFFFFFC00_00000000);
    let _result = epsilons.pattern_id_unchecked();
}

#[test]
fn test_pattern_id_unchecked_invalid() {
    let epsilons = PatternEpsilons(0x00000000_00000000);
    let _result = epsilons.pattern_id_unchecked();
}

#[test]
fn test_pattern_id_unchecked_no_pattern_id() {
    let epsilons = PatternEpsilons(0x00000000_003FFFFF - 1);
    let _result = epsilons.pattern_id_unchecked();
}

