// Answer 0

#[test]
fn test_is_word_start_ascii_at_zero() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"abc"; // haystack has at least one character
    let at = 0; // at == 0, should validate that word_before is false
    matcher.is_word_start_ascii(haystack, at);
}

#[test]
fn test_is_word_start_ascii_at_equals_haystack_len() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"abc"; // haystack has at least one character
    let at = haystack.len(); // at == haystack.len(), should validate that word_after is out of bounds
    matcher.is_word_start_ascii(haystack, at);
}

