// Answer 0

#[test]
fn test_case_insensitive_true() {
    let re_set = RegexSetBuilder::new(["foo(?-i:bar)quux"])
        .case_insensitive(true)
        .build()
        .unwrap();
}

#[test]
fn test_case_insensitive_false() {
    let re_set = RegexSetBuilder::new(["foo(?-i:bar)quux"])
        .case_insensitive(false)
        .build()
        .unwrap();
}

#[test]
fn test_case_insensitive_edge_case_empty() {
    let re_set = RegexSetBuilder::new([""])
        .case_insensitive(true)
        .build()
        .unwrap();
}

#[test]
fn test_case_insensitive_edge_case_multiple() {
    let re_set = RegexSetBuilder::new(["foo(?-i:bar)quux", "FOO(?-i:BAR)QUUX"])
        .case_insensitive(true)
        .build()
        .unwrap();
}

#[test]
fn test_case_insensitive_mixed_case() {
    let re_set = RegexSetBuilder::new(["fOo(?-i:bAr)qUux"])
        .case_insensitive(true)
        .build()
        .unwrap();
}

