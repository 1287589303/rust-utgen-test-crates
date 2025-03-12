// Answer 0

#[test]
fn test_word_end_half_case1() {
    let look = Look::WordEndHalf;
    let haystack = b"hello world";
    let at = 10; // Position after 'o' (non-word character)
    look.is_match(haystack, at);
}

#[test]
fn test_word_end_half_case2() {
    let look = Look::WordEndHalf;
    let haystack = b"test_case";
    let at = 9; // Position after '_' (non-word character)
    look.is_match(haystack, at);
}

#[test]
fn test_word_end_half_case3() {
    let look = Look::WordEndHalf;
    let haystack = b"abc-def";
    let at = 7; // Position after '-' (non-word character)
    look.is_match(haystack, at);
}

#[test]
fn test_word_end_half_case4() {
    let look = Look::WordEndHalf;
    let haystack = b"word#";
    let at = 5; // Position after '#' (non-word character)
    look.is_match(haystack, at);
}

