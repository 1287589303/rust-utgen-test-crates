// Answer 0

#[test]
fn test_multi_line_true() {
    let mut builder = TranslatorBuilder::new();
    builder.multi_line(true);
}

#[test]
fn test_multi_line_false() {
    let mut builder = TranslatorBuilder::new();
    builder.multi_line(false);
}

