// Answer 0

#[test]
fn test_is_word_end_ascii_at_zero() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b""; // empty haystack
    let at = 0;
    matcher.is_word_end_ascii(haystack, at);
}

#[test]
fn test_is_word_end_ascii_at_haystack_len() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b""; // empty haystack
    let at = haystack.len(); // at == len(haystack)
    matcher.is_word_end_ascii(haystack, at);
}

#[test]
fn test_is_word_end_ascii_non_word_byte_at_previous_position() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"."; // non-word byte before the specified position
    let at = 1; // at == haystack.len() - 1 (1) with non-word byte
    matcher.is_word_end_ascii(haystack, at);
}

