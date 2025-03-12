// Answer 0

#[test]
fn test_unicode_true() {
    let mut builder = RegexBuilder::new(r"\w");
    let _result = builder.unicode(true);
}

#[test]
fn test_unicode_false() {
    let mut builder = RegexBuilder::new(r"\w");
    let _result = builder.unicode(false);
}

