// Answer 0

#[test]
fn test_case_insensitive_true() {
    let mut builder = RegexBuilder::new("foo(?-i:bar)quux");
    builder.case_insensitive(true);
    let _ = builder.build().unwrap();
}

#[test]
fn test_case_insensitive_false() {
    let mut builder = RegexBuilder::new("foo(?-i:bar)quux");
    builder.case_insensitive(false);
    let _ = builder.build().unwrap();
}

#[test]
fn test_case_insensitive_empty_pattern() {
    let mut builder = RegexBuilder::new("");
    builder.case_insensitive(true);
    let _ = builder.build().unwrap();
}

#[test]
fn test_case_insensitive_custom_pattern() {
    let mut builder = RegexBuilder::new("custom_pattern");
    builder.case_insensitive(false);
    let _ = builder.build().unwrap();
}

