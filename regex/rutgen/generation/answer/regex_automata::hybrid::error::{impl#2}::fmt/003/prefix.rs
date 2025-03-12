// Answer 0

#[test]
fn test_fmt_insufficient_cache_capacity() {
    let minimum: usize = 1; // Minimum value to satisfy the condition
    let given: usize = 0; // Given value less than minimum
    
    let build_error = BuildError {
        kind: BuildErrorKind::InsufficientCacheCapacity { minimum, given },
    };
    
    let mut output = String::new();
    let _ = build_error.fmt(&mut output);
}

#[test]
fn test_fmt_insufficient_cache_capacity_boundary() {
    let minimum: usize = usize::MAX; // Maximum possible value for minimum
    let given: usize = usize::MAX - 1; // Given value, just below maximum
    
    let build_error = BuildError {
        kind: BuildErrorKind::InsufficientCacheCapacity { minimum, given },
    };
    
    let mut output = String::new();
    let _ = build_error.fmt(&mut output);
}

#[test]
fn test_fmt_insufficient_cache_capacity_large_values() {
    let minimum: usize = 1000; // A larger minimum value
    let given: usize = 999; // A valid 'given' value less than the minimum
    
    let build_error = BuildError {
        kind: BuildErrorKind::InsufficientCacheCapacity { minimum, given },
    };
    
    let mut output = String::new();
    let _ = build_error.fmt(&mut output);
}

