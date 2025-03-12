// Answer 0

#[test]
fn test_is_word_ascii_negate_at_0() {
    let haystack: &[u8] = b"hello";
    let matcher = LookMatcher::new();
    matcher.is_word_ascii_negate(haystack, 0);
}

#[test]
fn test_is_word_ascii_negate_at_1() {
    let haystack: &[u8] = b"hello";
    let matcher = LookMatcher::new();
    matcher.is_word_ascii_negate(haystack, 1);
}

#[test]
fn test_is_word_ascii_negate_at_2() {
    let haystack: &[u8] = b"hello";
    let matcher = LookMatcher::new();
    matcher.is_word_ascii_negate(haystack, 2);
}

#[test]
fn test_is_word_ascii_negate_at_3() {
    let haystack: &[u8] = b"hello";
    let matcher = LookMatcher::new();
    matcher.is_word_ascii_negate(haystack, 3);
}

#[test]
fn test_is_word_ascii_negate_at_4() {
    let haystack: &[u8] = b"hello";
    let matcher = LookMatcher::new();
    matcher.is_word_ascii_negate(haystack, 4);
}

#[test]
fn test_is_word_ascii_negate_at_length() {
    let haystack: &[u8] = b"hello";
    let matcher = LookMatcher::new();
    matcher.is_word_ascii_negate(haystack, haystack.len());
}

