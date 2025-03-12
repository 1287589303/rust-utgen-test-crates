// Answer 0

#[test]
fn test_insufficient_cache_capacity_zero_zero() {
    let minimum = 0;
    let given = 0;
    let _error = BuildError::insufficient_cache_capacity(minimum, given);
}

#[test]
fn test_insufficient_cache_capacity_zero_one() {
    let minimum = 0;
    let given = 1;
    let _error = BuildError::insufficient_cache_capacity(minimum, given);
}

#[test]
fn test_insufficient_cache_capacity_one_zero() {
    let minimum = 1;
    let given = 0;
    let _error = BuildError::insufficient_cache_capacity(minimum, given);
}

#[test]
fn test_insufficient_cache_capacity_one_one() {
    let minimum = 1;
    let given = 1;
    let _error = BuildError::insufficient_cache_capacity(minimum, given);
}

#[test]
fn test_insufficient_cache_capacity_one_two() {
    let minimum = 1;
    let given = 2;
    let _error = BuildError::insufficient_cache_capacity(minimum, given);
}

#[test]
fn test_insufficient_cache_capacity_max_max() {
    let minimum = std::usize::MAX;
    let given = std::usize::MAX;
    let _error = BuildError::insufficient_cache_capacity(minimum, given);
}

#[test]
fn test_insufficient_cache_capacity_max_zero() {
    let minimum = std::usize::MAX;
    let given = 0;
    let _error = BuildError::insufficient_cache_capacity(minimum, given);
}

#[test]
fn test_insufficient_cache_capacity_zero_max() {
    let minimum = 0;
    let given = std::usize::MAX;
    let _error = BuildError::insufficient_cache_capacity(minimum, given);
}

