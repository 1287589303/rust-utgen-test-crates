// Answer 0

#[test]
fn test_end_match_empty_haystack() {
    let look = Look::End;
    let haystack: Vec<u8> = vec![];
    let at = haystack.len();
    look.is_match(&haystack, at);
}

#[test]
fn test_end_match_non_empty_haystack() {
    let look = Look::End;
    let haystack = b"test string".to_vec();
    let at = haystack.len();
    look.is_match(&haystack, at);
}

