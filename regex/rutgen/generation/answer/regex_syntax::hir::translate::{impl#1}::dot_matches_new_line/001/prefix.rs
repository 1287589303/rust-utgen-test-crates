// Answer 0

#[test]
fn test_dot_matches_new_line_enable() {
    let mut builder = TranslatorBuilder::new();
    builder.dot_matches_new_line(true);
}

#[test]
fn test_dot_matches_new_line_multiple_calls() {
    let mut builder = TranslatorBuilder::new();
    builder.dot_matches_new_line(true).dot_matches_new_line(true);
}

