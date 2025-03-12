// Answer 0

#[test]
fn test_unicode_true() {
    let mut builder = Builder::new(vec!["test_pattern"]);
    builder.unicode(true);
}

#[test]
fn test_unicode_false() {
    let mut builder = Builder::new(vec!["test_pattern"]);
    builder.unicode(false);
}

