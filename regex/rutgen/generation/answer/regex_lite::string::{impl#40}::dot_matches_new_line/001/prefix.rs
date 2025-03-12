// Answer 0

#[test]
fn test_dot_matches_new_line_true() {
    let mut builder = RegexBuilder::new(r"foo.bar");
    builder.dot_matches_new_line(true);
}

#[test]
fn test_dot_matches_new_line_false() {
    let mut builder = RegexBuilder::new(r"foo.bar");
    builder.dot_matches_new_line(false);
}

#[test]
fn test_dot_matches_new_line_chain() {
    let mut builder = RegexBuilder::new(r"foo.bar");
    builder.dot_matches_new_line(true)
           .dot_matches_new_line(false);
}

