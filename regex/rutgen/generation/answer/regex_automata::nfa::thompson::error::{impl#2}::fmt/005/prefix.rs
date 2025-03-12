// Answer 0

#[test]
fn test_too_many_patterns() {
    let given = 2; // Example value, ensure 0 < given
    let limit = 1; // Example value, ensure 0 < limit and given > limit
    let error = BuildError::too_many_patterns(given);
    let mut f = core::fmt::Formatter::new();
    let _ = error.fmt(&mut f);
}

#[test]
fn test_exceeded_size_limit() {
    let limit = 1; // Example value, ensure 0 < limit
    let error = BuildError::exceeded_size_limit(limit);
    let mut f = core::fmt::Formatter::new();
    let _ = error.fmt(&mut f);
}

#[test]
fn test_invalid_capture_index() {
    let index = 1; // Example value, ensure index is valid (too big or discontinuous)
    let error = BuildError::invalid_capture_index(index);
    let mut f = core::fmt::Formatter::new();
    let _ = error.fmt(&mut f);
}

