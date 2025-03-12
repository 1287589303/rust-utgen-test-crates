// Answer 0

#[test]
fn test_unicode_true() {
    let mut builder = RegexSetBuilder::new(vec![r"\w"]);
    builder.unicode(true);
}

#[test]
fn test_unicode_false() {
    let mut builder = RegexSetBuilder::new(vec![r"\w"]);
    builder.unicode(false);
}

