// Answer 0

#[test]
fn test_is_word_start_unicode_valid_at_start() {
    let haystack = b"hello";
    let at = 0;
    let matcher = LookMatcher::new();
    let _ = matcher.is_word_start_unicode(haystack, at);
}

#[test]
fn test_is_word_start_unicode_valid_at_middle() {
    let haystack = b"hello world";
    let at = 6;
    let matcher = LookMatcher::new();
    let _ = matcher.is_word_start_unicode(haystack, at);
}

#[test]
fn test_is_word_start_unicode_valid_at_end() {
    let haystack = b"hello world";
    let at = 11; // at haystack.len()
    let matcher = LookMatcher::new();
    let _ = matcher.is_word_start_unicode(haystack, at);
}

