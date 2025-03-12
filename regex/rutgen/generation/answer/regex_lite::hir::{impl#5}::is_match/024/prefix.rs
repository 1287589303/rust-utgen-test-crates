// Answer 0

#[test]
fn test_is_match_word_negate_at_start_empty_haystack() {
    let look = Look::WordNegate;
    let haystack: &[u8] = &[];
    let at = 0;
    look.is_match(haystack, at);
}

#[test]
fn test_is_match_word_negate_at_end_empty_haystack() {
    let look = Look::WordNegate;
    let haystack: &[u8] = &[];
    let at = 0; // Equivalent to haystack.len()
    look.is_match(haystack, at);
}

