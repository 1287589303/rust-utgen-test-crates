// Answer 0

#[test]
fn test_find_at_valid_match_start_zero() {
    let re = Regex { pikevm: Arc::new(PikeVM::new(NFA::new())), pool: CachePool::new() };
    let haystack = "hello";
    let start = 0;
    re.find_at(haystack, start);
}

#[test]
fn test_find_at_valid_match_start_within_bounds() {
    let re = Regex { pikevm: Arc::new(PikeVM::new(NFA::new())), pool: CachePool::new() };
    let haystack = "hello world";
    let start = 6;
    re.find_at(haystack, start);
}

#[test]
fn test_find_at_valid_match_start_at_end() {
    let re = Regex { pikevm: Arc::new(PikeVM::new(NFA::new())), pool: CachePool::new() };
    let haystack = "regex";
    let start = 4; // Assuming a match at the last character
    re.find_at(haystack, start);
}

