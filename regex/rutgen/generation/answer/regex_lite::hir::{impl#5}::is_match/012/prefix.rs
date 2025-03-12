// Answer 0

#[test]
fn test_word_end_at_start() {
    let look = Look::WordEnd;
    let haystack: &[u8] = b"word";
    let at = 0;
    look.is_match(haystack, at);
}

#[test]
fn test_word_end_at_end() {
    let look = Look::WordEnd;
    let haystack: &[u8] = b"word";
    let at = 4; // haystack.len()
    look.is_match(haystack, at);
}

