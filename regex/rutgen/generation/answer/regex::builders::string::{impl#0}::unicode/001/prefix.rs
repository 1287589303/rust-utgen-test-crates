// Answer 0

#[test]
fn test_regex_builder_unicode_true() {
    let mut builder = RegexBuilder::new(r"\w");
    builder.unicode(true);
    let _ = builder.build().unwrap();
}

#[test]
fn test_regex_builder_unicode_false() {
    let mut builder = RegexBuilder::new(r"\w");
    builder.unicode(false);
    let _ = builder.build().unwrap();
}

