// Answer 0

#[test]
fn test_matches_inline_word_end_unicode_start() {
    let haystack: Vec<u8> = b"hello world".to_vec();
    let matcher = LookMatcher::new();
    let at = 11; // Point at the end of the "world"
    matcher.matches_inline(Look::WordEndUnicode, &haystack, at);
}

#[test]
fn test_matches_inline_word_end_unicode_middle() {
    let haystack: Vec<u8> = b"hello world".to_vec();
    let matcher = LookMatcher::new();
    let at = 5; // Point at a boundary before " "
    matcher.matches_inline(Look::WordEndUnicode, &haystack, at);
}

#[test]
fn test_matches_inline_word_end_unicode_valid_boundary() {
    let haystack: Vec<u8> = b"hello".to_vec();
    let matcher = LookMatcher::new();
    let at = 5; // Point at the end of "hello"
    matcher.matches_inline(Look::WordEndUnicode, &haystack, at);
}

#[test]
#[should_panic] // Assuming feature not enabled
fn test_matches_inline_word_end_unicode_out_of_bounds() {
    let haystack: Vec<u8> = b"hello".to_vec();
    let matcher = LookMatcher::new();
    let at = 6; // Out of bounds
    matcher.matches_inline(Look::WordEndUnicode, &haystack, at);
}

