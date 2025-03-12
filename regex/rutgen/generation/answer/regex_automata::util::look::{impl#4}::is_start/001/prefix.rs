// Answer 0

#[test]
fn test_is_start_zero() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"example";
    let result = matcher.is_start(haystack, 0);
}

#[test]
fn test_is_start_one() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"example";
    let result = matcher.is_start(haystack, 1);
}

#[test]
fn test_is_start_five() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"example";
    let result = matcher.is_start(haystack, 5);
}

#[test]
fn test_is_start_ten() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"example";
    let result = matcher.is_start(haystack, 10);
}

#[test]
fn test_is_start_fifteen() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"example";
    let result = matcher.is_start(haystack, 15);
}

#[test]
fn test_is_start_haystack_size() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"example";
    let result = matcher.is_start(haystack, haystack.len());
}

#[test]
#[should_panic] 
fn test_is_start_haystack_size_plus_one() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"example";
    let result = matcher.is_start(haystack, haystack.len() + 1);
}

