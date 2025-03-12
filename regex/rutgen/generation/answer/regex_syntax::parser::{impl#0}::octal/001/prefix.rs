// Answer 0

#[test]
fn test_octal_true() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.octal(true);
}

#[test]
fn test_octal_false() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.octal(false);
}

