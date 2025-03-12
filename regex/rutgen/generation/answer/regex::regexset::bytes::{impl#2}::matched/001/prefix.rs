// Answer 0

#[test]
fn test_matched_valid_index_0() {
    let set = RegexSet::new([
        r"[a-z]+@[a-z]+\.(com|org|net)",
        r"[a-z]+\.(com|org|net)",
    ]).unwrap();
    let matches = set.matches(b"example.com");
    let _ = matches.matched(0);
}

#[test]
fn test_matched_valid_index_1() {
    let set = RegexSet::new([
        r"[a-z]+@[a-z]+\.(com|org|net)",
        r"[a-z]+\.(com|org|net)",
    ]).unwrap();
    let matches = set.matches(b"example.com");
    let _ = matches.matched(1);
}

#[test]
#[should_panic]
fn test_matched_invalid_index_negative() {
    let set = RegexSet::new([
        r"[a-z]+@[a-z]+\.(com|org|net)",
        r"[a-z]+\.(com|org|net)",
    ]).unwrap();
    let matches = set.matches(b"example.com");
    let _ = matches.matched(usize::MAX);
}

#[test]
#[should_panic]
fn test_matched_invalid_index_equal_to_len() {
    let set = RegexSet::new([
        r"[a-z]+@[a-z]+\.(com|org|net)",
        r"[a-z]+\.(com|org|net)",
    ]).unwrap();
    let matches = set.matches(b"example.com");
    let _ = matches.matched(2);
}

