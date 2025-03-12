// Answer 0

#[test]
fn test_ignore_whitespace_true() {
    let mut builder = ParserBuilder::new();
    builder.ignore_whitespace(true);
}

#[test]
fn test_ignore_whitespace_false() {
    let mut builder = ParserBuilder::new();
    builder.ignore_whitespace(false);
}

