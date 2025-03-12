// Answer 0

#[test]
fn test_haystack_too_long_zero() {
    let error = MatchError::haystack_too_long(0);
}

#[test]
fn test_haystack_too_long_one() {
    let error = MatchError::haystack_too_long(1);
}

#[test]
fn test_haystack_too_long_ten() {
    let error = MatchError::haystack_too_long(10);
}

#[test]
fn test_haystack_too_long_one_hundred() {
    let error = MatchError::haystack_too_long(100);
}

#[test]
fn test_haystack_too_long_one_thousand() {
    let error = MatchError::haystack_too_long(1000);
}

#[test]
fn test_haystack_too_long_max_usize() {
    let error = MatchError::haystack_too_long(usize::MAX);
}

