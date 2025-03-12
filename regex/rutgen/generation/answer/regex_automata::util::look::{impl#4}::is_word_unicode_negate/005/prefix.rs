// Answer 0

#[test]
fn test_is_word_unicode_negate_valid_utf8() {
    let mut matcher = LookMatcher::new();
    let haystack = b"hello";
    let at = 1; 
    matcher.is_word_unicode_negate(haystack, at).unwrap();
}

#[test]
fn test_is_word_unicode_negate_invalid_utf8() {
    let mut matcher = LookMatcher::new();
    let haystack = b"\xFFhello";
    let at = 1; 
    matcher.is_word_unicode_negate(haystack, at).unwrap();
}

#[test]
fn test_is_word_unicode_negate_boundary_valid() {
    let mut matcher = LookMatcher::new();
    let haystack = b"hello\xFF";
    let at = 5; 
    matcher.is_word_unicode_negate(haystack, at).unwrap();
}

#[test]
fn test_is_word_unicode_negate_boundary_invalid() {
    let mut matcher = LookMatcher::new();
    let haystack = b"\xFF\xFF";
    let at = 2;  
    matcher.is_word_unicode_negate(haystack, at).unwrap();
}

#[test]
fn test_is_word_unicode_negate_empty_haystack() {
    let mut matcher = LookMatcher::new();
    let haystack = b"";
    let at = 0; 
    matcher.is_word_unicode_negate(haystack, at).unwrap();
}

