// Answer 0

#[test]
fn test_is_size_limit_exceeded_too_many_states() {
    use regex_automata::dfa::{BuildError, BuildErrorKind};

    let error = BuildError {
        kind: BuildErrorKind::TooManyStates { 
            given: 100, 
            limit: 50 
        },
    };

    let result = error.is_size_limit_exceeded();
}

#[test]
fn test_is_size_limit_exceeded_dfa_exceeded_size_limit() {
    use regex_automata::dfa::{BuildError, BuildErrorKind};

    let error = BuildError {
        kind: BuildErrorKind::DFAExceededSizeLimit { 
            limit: 1024 
        },
    };

    let result = error.is_size_limit_exceeded();
}

#[test]
fn test_is_size_limit_exceeded_determinize_exceeded_size_limit() {
    use regex_automata::dfa::{BuildError, BuildErrorKind};

    let error = BuildError {
        kind: BuildErrorKind::DeterminizeExceededSizeLimit { 
            limit: 2048 
        },
    };

    let result = error.is_size_limit_exceeded();
}

#[test]
fn test_is_size_limit_exceeded_too_many_start_states() {
    use regex_automata::dfa::{BuildError, BuildErrorKind};

    let error = BuildError {
        kind: BuildErrorKind::TooManyStartStates,
    };

    let result = error.is_size_limit_exceeded();
}

#[test]
fn test_is_size_limit_exceeded_too_many_match_pattern_ids() {
    use regex_automata::dfa::{BuildError, BuildErrorKind};

    let error = BuildError {
        kind: BuildErrorKind::TooManyMatchPatternIDs,
    };

    let result = error.is_size_limit_exceeded();
}

