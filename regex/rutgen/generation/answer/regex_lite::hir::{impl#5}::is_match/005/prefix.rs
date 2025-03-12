// Answer 0

#[test]
fn test_word_end_case_1() {
    let look = Look::WordEnd;
    let haystack: &[u8] = b"hello world";
    let at = 5; // 'o' -> ' ' transition
    look.is_match(haystack, at);
}

#[test]
fn test_word_end_case_2() {
    let look = Look::WordEnd;
    let haystack: &[u8] = b"hello_world!";
    let at = 11; // '_' -> '!' transition
    look.is_match(haystack, at);
}

#[test]
fn test_word_end_case_3() {
    let look = Look::WordEnd;
    let haystack: &[u8] = b"test_case123";
    let at = 10; // '3' -> end transition
    look.is_match(haystack, at);
}

#[test]
fn test_word_end_case_4() {
    let look = Look::WordEnd;
    let haystack: &[u8] = b"goodbye_world";
    let at = 12; // 'd' -> end transition
    look.is_match(haystack, at);
}

