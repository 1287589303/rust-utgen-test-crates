// Answer 0

#[test]
fn test_word_end_half_at_end_of_non_word_byte() {
    let haystack: &[u8] = b"abc"; // Haystack with non-word bytes
    let look = Look::WordEndHalf;
    let at = haystack.len(); // at is equal to the length of the haystack

    look.is_match(haystack, at);
}

#[test]
fn test_word_end_half_at_end_of_empty() {
    let haystack: &[u8] = b""; // Empty haystack
    let look = Look::WordEndHalf;
    let at = haystack.len(); // at is equal to the length of the haystack

    look.is_match(haystack, at);
}

#[test]
fn test_word_end_half_at_end_of_word() {
    let haystack: &[u8] = b"word"; // Haystack with word at end
    let look = Look::WordEndHalf;
    let at = haystack.len(); // at is equal to the length of the haystack

    look.is_match(haystack, at);
}

