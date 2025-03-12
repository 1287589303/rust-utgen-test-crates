// Answer 0

#[test]
fn test_word_start_true_case() {
    let look = Look::WordStart;
    let haystack: &[u8] = b" hello";
    let at = 1; // at > 0 and at < haystack.len()
    let result = look.is_match(haystack, at);
}

#[test]
fn test_word_start_boundary_case() {
    let look = Look::WordStart;
    let haystack: &[u8] = b"hello ";
    let at = 5; // at > 0 and at < haystack.len()
    let result = look.is_match(haystack, at);
}

