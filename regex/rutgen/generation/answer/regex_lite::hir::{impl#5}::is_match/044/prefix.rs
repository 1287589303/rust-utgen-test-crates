// Answer 0

#[test]
fn test_is_match_start() {
    let haystack: Vec<u8> = vec![b'a', b'b', b'c'];
    let look = Look::Start;
    let _ = look.is_match(&haystack, 0);
}

#[test]
fn test_is_match_start_empty() {
    let haystack: Vec<u8> = vec![];
    let look = Look::Start;
    let _ = look.is_match(&haystack, 0);
}

