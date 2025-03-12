// Answer 0

#[test]
fn test_is_word_end_ascii_case1() {
    let haystack = b"hello_world";
    let at = 11; // at is equal to haystack.len(), should not panic and return false
    let matcher = LookMatcher::new();
    matcher.is_word_end_ascii(haystack, at);
}

#[test]
fn test_is_word_end_ascii_case2() {
    let haystack = b"hello_world";
    let at = 10; // at < haystack.len() and haystack[at - 1] is a word byte ('d'), haystack[at] is not a word byte ('\0')
    let matcher = LookMatcher::new();
    matcher.is_word_end_ascii(haystack, at);
}

#[test]
fn test_is_word_end_ascii_case3() {
    let haystack = b"hello_world!";
    let at = 11; // at < haystack.len() (11) and haystack[at - 1] is a word byte ('d'), haystack[at] is '!', not a word byte
    let matcher = LookMatcher::new();
    matcher.is_word_end_ascii(haystack, at);
}

#[test]
fn test_is_word_end_ascii_case4() {
    let haystack = b"hello_";
    let at = 6; // at < haystack.len() (6) and haystack[at - 1] is a word byte ('_'), haystack[at] is not a word byte (out of bounds)
    let matcher = LookMatcher::new();
    matcher.is_word_end_ascii(haystack, at);
}

