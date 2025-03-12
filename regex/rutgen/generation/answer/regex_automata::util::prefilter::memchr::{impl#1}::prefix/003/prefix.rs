// Answer 0

#[test]
fn test_prefix_returns_none_when_character_does_not_match() {
    let haystack: &[u8] = b"hello";
    let span = Span { start: 0, end: 1 }; // haystack[0] is 'h'
    let prefilter = Memchr(b'e'); // self.0 is 'e', which does not match haystack[0]
    let result = prefilter.prefix(haystack, span);
}

#[test]
fn test_prefix_returns_none_for_different_start_index() {
    let haystack: &[u8] = b"world";
    let span = Span { start: 1, end: 2 }; // haystack[1] is 'o'
    let prefilter = Memchr(b'x'); // self.0 is 'x', which does not match haystack[1]
    let result = prefilter.prefix(haystack, span);
}

#[test]
fn test_prefix_returns_none_when_last_character_does_not_match() {
    let haystack: &[u8] = b"foo";
    let span = Span { start: 2, end: 3 }; // haystack[2] is 'o'
    let prefilter = Memchr(b'a'); // self.0 is 'a', which does not match haystack[2]
    let result = prefilter.prefix(haystack, span);
}

#[test]
fn test_prefix_returns_none_with_mid_character_mismatch() {
    let haystack: &[u8] = b"example";
    let span = Span { start: 3, end: 4 }; // haystack[3] is 'm'
    let prefilter = Memchr(b'n'); // self.0 is 'n', which does not match haystack[3]
    let result = prefilter.prefix(haystack, span);
}

