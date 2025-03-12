// Answer 0

#[test]
fn test_too_many_match_pattern_ids() {
    let error = BuildError::too_many_match_pattern_ids();
}

#[test]
fn test_too_many_match_pattern_ids_return_type() {
    let error: BuildError = BuildError::too_many_match_pattern_ids();
}

