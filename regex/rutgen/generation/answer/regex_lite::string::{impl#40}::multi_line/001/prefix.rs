// Answer 0

#[test]
fn test_multi_line_true() {
    let mut builder = RegexBuilder::new(r"^foo$");
    builder.multi_line(true);
}

#[test]
fn test_multi_line_false() {
    let mut builder = RegexBuilder::new(r"^foo$");
    builder.multi_line(false);
}

#[test]
fn test_multi_line_with_different_pattern() {
    let mut builder = RegexBuilder::new(r"^bar$");
    builder.multi_line(true);
}

#[test]
fn test_multi_line_edge_case_empty_pattern() {
    let mut builder = RegexBuilder::new(r"");
    builder.multi_line(false);
}

#[test]
fn test_multi_line_with_complex_pattern() {
    let mut builder = RegexBuilder::new(r"^(foo|bar)$");
    builder.multi_line(true);
}

