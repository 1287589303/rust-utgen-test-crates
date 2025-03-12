// Answer 0

#[test]
fn test_is_match_word_start_empty_haystack() {
    let look = Look::Word;
    let haystack: &[u8] = &[];
    let at = 0;
    look.is_match(haystack, at);
}

#[test]
fn test_is_match_word_negate_empty_haystack() {
    let look = Look::WordNegate;
    let haystack: &[u8] = &[];
    let at = 0;
    look.is_match(haystack, at);
} 

#[test]
fn test_is_match_word_start_half_empty_haystack() {
    let look = Look::WordStartHalf;
    let haystack: &[u8] = &[];
    let at = 0;
    look.is_match(haystack, at);
}

#[test]
fn test_is_match_word_end_half_empty_haystack() {
    let look = Look::WordEndHalf;
    let haystack: &[u8] = &[];
    let at = 0;
    look.is_match(haystack, at);
}

