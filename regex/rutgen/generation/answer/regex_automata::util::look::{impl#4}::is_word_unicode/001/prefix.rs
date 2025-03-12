// Answer 0

#[test]
fn test_is_word_unicode_err_at_zero() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"";
    let at: usize = 0;
    let _ = matcher.is_word_unicode(haystack, at);
}

#[test]
fn test_is_word_unicode_err_middle() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"hello";
    let at: usize = 3;
    let _ = matcher.is_word_unicode(haystack, at);
}

#[test]
fn test_is_word_unicode_err_at_length() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"world";
    let at: usize = haystack.len();
    let _ = matcher.is_word_unicode(haystack, at);
}

#[test]
fn test_is_word_unicode_err_non_empty_haystack_at_one() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"word";
    let at: usize = 1;
    let _ = matcher.is_word_unicode(haystack, at);
}

