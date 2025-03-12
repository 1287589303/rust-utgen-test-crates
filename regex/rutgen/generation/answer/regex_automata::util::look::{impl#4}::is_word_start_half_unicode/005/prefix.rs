// Answer 0

#[test]
fn test_is_word_start_half_unicode_at_zero_empty_haystack() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = &[];
    let at = 0;
    matcher.is_word_start_half_unicode(haystack, at).unwrap();
}

#[test]
fn test_is_word_start_half_unicode_at_zero_non_empty_haystack() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"valid";
    let at = 0;
    matcher.is_word_start_half_unicode(haystack, at).unwrap();
}

