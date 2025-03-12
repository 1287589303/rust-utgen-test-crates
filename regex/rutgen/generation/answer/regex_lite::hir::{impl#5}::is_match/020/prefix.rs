// Answer 0

#[test]
fn test_word_start_case1() {
    let look = Look::WordStart;
    let haystack: &[u8] = b" _"; // Non-word character before a word character
    let at = 0;
    look.is_match(haystack, at);
}

#[test]
fn test_word_start_case2() {
    let look = Look::WordStart;
    let haystack: &[u8] = b"@abc"; // Non-word character at the start 
    let at = 0;
    look.is_match(haystack, at);
}

#[test]
fn test_word_start_case3() {
    let look = Look::WordStart;
    let haystack: &[u8] = b" ."; // Non-word character before a word character
    let at = 0;
    look.is_match(haystack, at);
}

#[test]
fn test_word_start_case4() {
    let look = Look::WordStart;
    let haystack: &[u8] = b"123abc"; // Non-word character at start
    let at = 0;
    look.is_match(haystack, at);
}

#[test]
fn test_word_start_bound() {
    let look = Look::WordStart;
    let haystack: &[u8] = b""; // Empty haystack
    let at = 0; // at should be 0 but haystack length is also 0
    look.is_match(haystack, at);
}

