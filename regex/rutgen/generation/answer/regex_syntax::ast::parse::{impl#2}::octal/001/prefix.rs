// Answer 0

#[test]
fn test_octal_true() {
    let mut builder = ParserBuilder::new();
    builder.octal(true);
}

#[test]
fn test_octal_false() {
    let mut builder = ParserBuilder::new();
    builder.octal(false);
}

