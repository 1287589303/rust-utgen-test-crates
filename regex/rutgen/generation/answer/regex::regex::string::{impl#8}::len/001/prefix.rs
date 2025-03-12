// Answer 0

#[test]
fn test_len_basic() {
    let haystack = "hello";
    let m = Match::new(haystack, 1, 4);
    let length = m.len();
}

#[test]
fn test_len_single_character() {
    let haystack = "a";
    let m = Match::new(haystack, 0, 1);
    let length = m.len();
}

#[test]
fn test_len_entire_string() {
    let haystack = "world";
    let m = Match::new(haystack, 0, 5);
    let length = m.len();
}

#[test]
fn test_len_substring() {
    let haystack = "regex";
    let m = Match::new(haystack, 1, 3);
    let length = m.len();
}

#[test]
fn test_len_empty_match() {
    let haystack = "nonempty";
    let m = Match::new(haystack, 3, 3);
    let length = m.len();
}

