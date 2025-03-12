// Answer 0

#[test]
fn test_dot_matches_new_line_true() {
    let mut regex_builder = RegexBuilder::new(r"foo.bar");
    regex_builder.dot_matches_new_line(true);
}

#[test]
fn test_dot_matches_new_line_false() {
    let mut regex_builder = RegexBuilder::new(r"foo.bar");
    regex_builder.dot_matches_new_line(false);
}

#[test]
fn test_dot_matches_new_line_edge_case() {
    let mut regex_builder = RegexBuilder::new(r"foo.bar");
    regex_builder.dot_matches_new_line(true);
    regex_builder.dot_matches_new_line(false);
}

