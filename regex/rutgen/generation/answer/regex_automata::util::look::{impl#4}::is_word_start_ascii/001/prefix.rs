// Answer 0

#[test]
fn test_is_word_start_ascii_case1() {
    let haystack = b"hello_world_this";
    let at = 5; // at > 0 and at < haystack.len()
    let matcher = LookMatcher::new();
    matcher.is_word_start_ascii(haystack, at);
}

#[test]
fn test_is_word_start_ascii_case2() {
    let haystack = b"1two_three";
    let at = 3; // at > 0 and at < haystack.len()
    let matcher = LookMatcher::new();
    matcher.is_word_start_ascii(haystack, at);
}

#[test]
fn test_is_word_start_ascii_case3() {
    let haystack = b"abc_def";
    let at = 3; // at > 0 and at < haystack.len()
    let matcher = LookMatcher::new();
    matcher.is_word_start_ascii(haystack, at);
}

