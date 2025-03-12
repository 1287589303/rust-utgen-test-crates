// Answer 0

#[test]
fn test_matched_any_non_empty_patterns() {
    let patterns = vec![
        r"[a-z]+@[a-z]+\.(com|org|net)",
    ];
    let set = SetMatches(PatternSet::new(patterns).unwrap());
    let result = set.matched_any();
}

#[test]
fn test_matched_any_empty_patterns() {
    let patterns: Vec<&str> = vec![];
    let set = SetMatches(PatternSet::new(patterns).unwrap());
    let result = set.matched_any();
}

#[test]
fn test_matched_any_mixed_patterns() {
    let patterns = vec![
        r"[a-z]+@[a-z]+\.(com|org|net)",
        r"[0-9]{3}", // invalid pattern that won't match any string
    ];
    let set = SetMatches(PatternSet::new(patterns).unwrap());
    let result = set.matched_any();
}

#[test]
fn test_matched_any_matching_pattern() {
    let patterns = vec![
        r"[a-z]+@[a-z]+\.(com|org|net)",
    ];
    let set = SetMatches(PatternSet::new(patterns).unwrap());
    let input = b"foo@example.com"; // binary string that matches the regex
    let matches = set.matches(input);
    let result = matches.matched_any();
}

#[test]
fn test_matched_any_no_matching_pattern() {
    let patterns = vec![
        r"[0-9]{3}",
    ];
    let set = SetMatches(PatternSet::new(patterns).unwrap());
    let input = b"foo@example.com"; // binary string that does not match the regex
    let matches = set.matches(input);
    let result = matches.matched_any();
}

