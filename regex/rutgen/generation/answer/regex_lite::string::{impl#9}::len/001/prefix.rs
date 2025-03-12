// Answer 0

#[test]
fn test_match_len_empty() {
    let haystack = "test";
    let m = Match::new(haystack, 2, 2);
    let _len = m.len();
}

#[test]
fn test_match_len_single_character() {
    let haystack = "abc";
    let m = Match::new(haystack, 1, 2);
    let _len = m.len();
}

#[test]
fn test_match_len_multiple_characters() {
    let haystack = "abcdef";
    let m = Match::new(haystack, 1, 4);
    let _len = m.len();
}

#[test]
fn test_match_len_full_length() {
    let haystack = "hello";
    let m = Match::new(haystack, 0, 5);
    let _len = m.len();
}

#[test]
fn test_match_len_zero_length() {
    let haystack = "world";
    let m = Match::new(haystack, 3, 3);
    let _len = m.len();
}

