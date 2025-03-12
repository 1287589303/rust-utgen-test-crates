// Answer 0

#[test]
fn test_is_word_ascii_case1() {
    let mut matcher = LookMatcher::new();
    let haystack = [b'a', b' ', b'_', b'9', b'!', b'Z'];
    let at = 3; // At b'9', which is a word byte
    matcher.is_word_ascii(&haystack, at);
}

#[test]
fn test_is_word_ascii_case2() {
    let mut matcher = LookMatcher::new();
    let haystack = [b'a', b' ', b'_', b'9', b'!', b'Z'];
    let at = 2; // At b'_', which is a word byte
    matcher.is_word_ascii(&haystack, at);
}

#[test]
fn test_is_word_ascii_case3() {
    let mut matcher = LookMatcher::new();
    let haystack = [b'a', b' ', b'_', b'9', b'!', b'Z'];
    let at = 1; // At b' ', which is not a word byte
    matcher.is_word_ascii(&haystack, at);
}

#[test]
fn test_is_word_ascii_case4() {
    let mut matcher = LookMatcher::new();
    let haystack = [b'a', b' ', b'_', b'9', b'!', b'Z'];
    let at = 0; // At b'a', which is a word byte
    matcher.is_word_ascii(&haystack, at);
}

