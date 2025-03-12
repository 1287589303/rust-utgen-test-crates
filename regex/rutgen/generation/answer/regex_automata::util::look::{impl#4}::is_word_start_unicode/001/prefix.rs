// Answer 0

#[test]
fn test_is_word_start_unicode_empty_haystack_at_0() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = &[];
    let at = 0;
    matcher.is_word_start_unicode(haystack, at).unwrap_err();
}

#[test]
fn test_is_word_start_unicode_empty_haystack_at_1() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = &[];
    let at = 1;
    matcher.is_word_start_unicode(haystack, at).unwrap_err();
}

