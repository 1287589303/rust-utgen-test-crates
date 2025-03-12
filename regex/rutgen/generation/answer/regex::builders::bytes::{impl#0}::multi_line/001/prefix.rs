// Answer 0

#[test]
fn test_multi_line_true() {
    let pattern = r"^foo$";
    let mut builder = RegexBuilder::new(pattern);
    builder.multi_line(true);
}

#[test]
fn test_multi_line_false() {
    let pattern = r"^bar$";
    let mut builder = RegexBuilder::new(pattern);
    builder.multi_line(false);
}

#[test]
fn test_multi_line_with_special_chars() {
    let pattern = r"^\d{3}-\d{2}-\d{4}$";
    let mut builder = RegexBuilder::new(pattern);
    builder.multi_line(true);
}

#[test]
fn test_multi_line_empty_string() {
    let pattern = r"";
    let mut builder = RegexBuilder::new(pattern);
    builder.multi_line(false);
}

