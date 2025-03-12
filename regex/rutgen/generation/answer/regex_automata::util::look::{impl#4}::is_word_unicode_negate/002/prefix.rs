// Answer 0

#[test]
fn test_is_word_unicode_negate_valid_before_invalid() {
    let mut matcher = LookMatcher::new();
    let haystack: &[u8] = b"hello\xFF"; // valid UTF-8 before invalid byte
    let at: usize = 5; // at > 0
    matcher.is_word_unicode_negate(haystack, at).unwrap();
}

#[test]
fn test_is_word_unicode_negate_invalid_before() {
    let mut matcher = LookMatcher::new();
    let haystack: &[u8] = b"\xFFworld"; // invalid UTF-8 before valid byte
    let at: usize = 0; // at = 0
    matcher.is_word_unicode_negate(haystack, at).unwrap();
}

#[test]
fn test_is_word_unicode_negate_invalid_utf8() {
    let mut matcher = LookMatcher::new();
    let haystack: &[u8] = b"\xFF\xFF"; // invalid UTF-8 sequence
    let at: usize = 1; // at > 0
    matcher.is_word_unicode_negate(haystack, at).unwrap();
}

#[test]
fn test_is_word_unicode_negate_empty_haystack() {
    let mut matcher = LookMatcher::new();
    let haystack: &[u8] = b""; // empty haystack
    let at: usize = 0; // at = 0
    matcher.is_word_unicode_negate(haystack, at).unwrap();
}

#[test]
fn test_is_word_unicode_negate_valid_after() {
    let mut matcher = LookMatcher::new();
    let haystack: &[u8] = b"hello"; // valid UTF-8 only
    let at: usize = 5; // at > 0
    matcher.is_word_unicode_negate(haystack, at).unwrap();
}

