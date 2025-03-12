// Answer 0

#[test]
fn test_is_word_start_half_ascii_with_word_byte() {
    let matcher = LookMatcher::new();
    let haystack = b"word_example";
    let at = 1; // at > 0
    let result = matcher.is_word_start_half_ascii(haystack, at);
}

#[test]
fn test_is_word_start_half_ascii_with_non_word_byte() {
    let matcher = LookMatcher::new();
    let haystack = b"!nonword";
    let at = 1; // at > 0
    let result = matcher.is_word_start_half_ascii(haystack, at);
}

#[test]
fn test_is_word_start_half_ascii_with_first_byte_word() {
    let matcher = LookMatcher::new();
    let haystack = b"1_first";
    let at = 1; // at > 0
    let result = matcher.is_word_start_half_ascii(haystack, at);
}

#[test]
fn test_is_word_start_half_ascii_with_first_byte_non_word() {
    let matcher = LookMatcher::new();
    let haystack = b"@starts";
    let at = 1; // at > 0
    let result = matcher.is_word_start_half_ascii(haystack, at);
}

