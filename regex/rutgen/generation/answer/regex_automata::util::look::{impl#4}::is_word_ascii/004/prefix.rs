// Answer 0

#[test]
fn test_is_word_ascii_empty_haystack_at_zero() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = &[];
    let at = 0;
    matcher.is_word_ascii(haystack, at);
}

#[test]
fn test_is_word_ascii_single_word_byte_at_zero() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = &[b'a'];
    let at = 0;
    matcher.is_word_ascii(haystack, at);
}

#[test]
fn test_is_word_ascii_single_word_byte_at_len() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = &[b'a'];
    let at = haystack.len();
    matcher.is_word_ascii(haystack, at);
}

#[test]
fn test_is_word_ascii_empty_haystack_at_len() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = &[];
    let at = haystack.len();
    matcher.is_word_ascii(haystack, at);
}

