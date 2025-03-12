// Answer 0

#[test]
fn test_valid_regex_patterns() {
    let patterns = vec![
        "^[a-zA-Z0-9]+$",
        "\\d{3}-\\d{2}-\\d{4}",
        "[a-z]+",
        ".*"
    ];

    for pattern in patterns {
        let _ = Regex::from_str(pattern).unwrap();
    }
}

#[test]
fn test_invalid_regex_patterns() {
    let patterns = vec![
        "[a-b",
        "(a|b",
        "(*invalid)",
        ""
    ];

    for pattern in patterns {
        let _ = Regex::from_str(pattern).unwrap_err();
    }
}

#[test]
#[should_panic]
fn test_exceeding_size_limit_pattern() {
    let large_pattern = "a{100000}";
    let _ = Regex::from_str(large_pattern).unwrap_err();
}

