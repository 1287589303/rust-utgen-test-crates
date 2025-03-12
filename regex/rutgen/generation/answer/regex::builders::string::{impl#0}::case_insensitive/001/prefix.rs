// Answer 0

#[test]
fn test_case_insensitive_true() {
    let re = RegexBuilder::new(r"foo(?-i:bar)quux")
        .case_insensitive(true)
        .build()
        .unwrap();
}

#[test]
fn test_case_insensitive_false() {
    let re = RegexBuilder::new(r"foo(?-i:bar)quux")
        .case_insensitive(false)
        .build()
        .unwrap();
}

#[test]
fn test_case_insensitive_edge_case() {
    let re = RegexBuilder::new("a")
        .case_insensitive(true)
        .build()
        .unwrap();
}

