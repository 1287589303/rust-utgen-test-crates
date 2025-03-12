// Answer 0

#[test]
fn test_is_utf8_true_patterns() {
    let test_patterns = [
        "a",
        "[^a]",
        ".",
        "\\W",
        "\\b",
        "\\B",
        "(?-u)\\b",
        "(?-u)\\B",
        "\\xFF",
    ];

    for pattern in &test_patterns {
        let properties = regex_syntax::ParserBuilder::new()
            .utf8(true)
            .build()
            .parse(pattern)
            .unwrap()
            .properties();
        properties.is_utf8();
    }
}

#[test]
fn test_is_utf8_false_patterns() {
    let test_patterns = [
        "(?-u)[^a]",
        "(?-u).",
        "(?-u)\\W",
        "(?-u)\\xFF",
    ];

    for pattern in &test_patterns {
        let properties = regex_syntax::ParserBuilder::new()
            .utf8(false)
            .build()
            .parse(pattern)
            .unwrap()
            .properties();
        properties.is_utf8();
    }
}

