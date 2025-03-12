// Answer 0

#[test]
fn test_matches_start() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"";
    matcher.matches(Look::Start, haystack, 0);
}

#[test]
fn test_matches_end() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"Hello";
    matcher.matches(Look::End, haystack, 5);
}

#[test]
fn test_matches_start_lf() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"\nHello";
    matcher.matches(Look::StartLF, haystack, 0);
}

#[test]
fn test_matches_end_lf() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"Hello\n";
    matcher.matches(Look::EndLF, haystack, 6);
}

#[test]
fn test_matches_word_ascii() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"Hello world";
    matcher.matches(Look::WordAscii, haystack, 5);
}

#[test]
fn test_matches_word_ascii_negate() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"Hello world";
    matcher.matches(Look::WordAsciiNegate, haystack, 6);
}

#[test]
fn test_matches_word_unicode() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = "你好 world".as_bytes();
    matcher.matches(Look::WordUnicode, haystack, 5);
}

#[test]
fn test_matches_word_unicode_negate() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = "你好 world".as_bytes();
    matcher.matches(Look::WordUnicodeNegate, haystack, 6);
}

#[test]
fn test_matches_start_crlf() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"\r\nHello";
    matcher.matches(Look::StartCRLF, haystack, 0);
}

#[test]
fn test_matches_end_crlf() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"Hello\r\n";
    matcher.matches(Look::EndCRLF, haystack, 7);
}

#[test]
fn test_matches_word_start_ascii() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b" Hello";
    matcher.matches(Look::WordStartAscii, haystack, 1);
}

#[test]
fn test_matches_word_end_ascii() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"Hello ";
    matcher.matches(Look::WordEndAscii, haystack, 5);
}

#[test]
fn test_matches_word_start_unicode() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = "你好world".as_bytes();
    matcher.matches(Look::WordStartUnicode, haystack, 0);
}

#[test]
fn test_matches_word_end_unicode() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = "Hello你好".as_bytes();
    matcher.matches(Look::WordEndUnicode, haystack, 5);
}

#[test]
fn test_matches_word_start_half_ascii() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b" Hello";
    matcher.matches(Look::WordStartHalfAscii, haystack, 0);
}

#[test]
fn test_matches_word_end_half_ascii() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = b"Hello ";
    matcher.matches(Look::WordEndHalfAscii, haystack, 5);
}

#[test]
fn test_matches_word_start_half_unicode() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = "你好world".as_bytes();
    matcher.matches(Look::WordStartHalfUnicode, haystack, 0);
}

#[test]
fn test_matches_word_end_half_unicode() {
    let matcher = LookMatcher::new();
    let haystack: &[u8] = "Hello你好".as_bytes();
    matcher.matches(Look::WordEndHalfUnicode, haystack, 5);
}

