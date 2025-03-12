// Answer 0

#[test]
fn test_is_word_start_half_unicode_valid_utf8() {
    let matcher = LookMatcher::new();
    let haystack: Vec<u8> = vec![b'T', b'e', b's', b't', b' ', b'w', b'o', b'r', b'd'];
    let at = 6; // valid UTF-8 character starting at index 6
    let _ = matcher.is_word_start_half_unicode(&haystack, at);
}

#[test]
fn test_is_word_start_half_unicode_utf8_error() {
    let matcher = LookMatcher::new();
    let haystack: Vec<u8> = vec![0xFF, 0xFE, 0xFD]; // invalid UTF-8 sequence
    let at = 3; // at beyond last valid byte to cause error
    let _ = matcher.is_word_start_half_unicode(&haystack, at);
}

#[test]
fn test_is_word_start_half_unicode_boundary_case() {
    let matcher = LookMatcher::new();
    let haystack: Vec<u8> = vec![b'a', b'1', b'!']; // valid ASCII only
    let at = 3; // valid position at the end
    let _ = matcher.is_word_start_half_unicode(&haystack, at);
}

#[test]
fn test_is_word_start_half_unicode_empty_haystack() {
    let matcher = LookMatcher::new();
    let haystack: Vec<u8> = vec![]; // empty byte array
    let at = 0; // at position 0 
    let _ = matcher.is_word_start_half_unicode(&haystack, at);
}

