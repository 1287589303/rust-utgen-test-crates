// Answer 0

#[test]
fn test_matches_inline_word_end_half_unicode_valid() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"hello world";
    let at = haystack.len(); // at should be at the end
    matcher.matches_inline(Look::WordEndHalfUnicode, haystack, at);
}

#[test]
fn test_matches_inline_word_end_half_unicode_zero_boundary() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"";
    let at = 0; // test at the start of an empty haystack
    matcher.matches_inline(Look::WordEndHalfUnicode, haystack, at);
}

#[test]
fn test_matches_inline_word_end_half_unicode_one_byte() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"a"; // single ASCII character
    let at = 1; // test at the end, should be a half-word boundary
    matcher.matches_inline(Look::WordEndHalfUnicode, haystack, at);
}

#[test]
fn test_matches_inline_word_end_half_unicode_unicode_char() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = "你好世界".as_bytes(); // valid Unicode characters
    let at = haystack.len(); // at should be at the end
    matcher.matches_inline(Look::WordEndHalfUnicode, haystack, at);
}

