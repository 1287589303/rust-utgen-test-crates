// Answer 0

#[test]
fn test_exceeded_size_limit_zero() {
    let limit = 0;
    let error = BuildError::exceeded_size_limit(limit);
}

#[test]
fn test_exceeded_size_limit_one() {
    let limit = 1;
    let error = BuildError::exceeded_size_limit(limit);
}

#[test]
fn test_exceeded_size_limit_max_usize() {
    let limit = std::usize::MAX;
    let error = BuildError::exceeded_size_limit(limit);
}

