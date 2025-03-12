// Answer 0

#[test]
fn test_case_insensitive_true() {
    let re = RegexSetBuilder::new(["foo(?-i:bar)quux"])
        .case_insensitive(true)
        .build()
        .unwrap();
}

#[test]
fn test_case_insensitive_false() {
    let re = RegexSetBuilder::new(["foo(?-i:bar)quux"])
        .case_insensitive(false)
        .build()
        .unwrap();
}

#[test]
fn test_case_insensitive_with_pattern() {
    let re = RegexSetBuilder::new(["(?i)foo", "(?i)bar"])
        .case_insensitive(true)
        .build()
        .unwrap();
}

#[test]
fn test_case_sensitive_pattern() {
    let re = RegexSetBuilder::new(["foo", "(?i:bar)quux"])
        .case_insensitive(false)
        .build()
        .unwrap();
}

#[test]
fn test_case_insensitive_combined_patterns() {
    let re = RegexSetBuilder::new(["(?i)foo", "(?-i:bar)quux"])
        .case_insensitive(true)
        .build()
        .unwrap();
}

