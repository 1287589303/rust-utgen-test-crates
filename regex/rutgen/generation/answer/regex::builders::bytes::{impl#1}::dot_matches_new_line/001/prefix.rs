// Answer 0

#[test]
fn test_dot_matches_new_line_true_with_single_pattern() {
    let mut builder = RegexSetBuilder::new(["foo.bar"]);
    let re = builder.dot_matches_new_line(true).build().unwrap();
    let hay = b"foo\nbar";
    re.is_match(hay);
}

#[test]
fn test_dot_matches_new_line_false_with_single_pattern() {
    let mut builder = RegexSetBuilder::new(["foo.bar"]);
    let re = builder.dot_matches_new_line(false).build().unwrap();
    let hay = b"foo\nbar";
    re.is_match(hay);
}

#[test]
fn test_dot_matches_new_line_true_with_multiple_patterns() {
    let mut builder = RegexSetBuilder::new(["foo.bar", "baz.qux"]);
    let re = builder.dot_matches_new_line(true).build().unwrap();
    let hay = b"foo\nbar baz\nqux";
    re.is_match(hay);
}

#[test]
fn test_dot_matches_new_line_false_with_empty_pattern() {
    let mut builder = RegexSetBuilder::new([""]);
    let re = builder.dot_matches_new_line(false).build().unwrap();
    let hay = b"";
    re.is_match(hay);
}

#[test]
fn test_dot_matches_new_line_true_with_new_line_in_pattern() {
    let mut builder = RegexSetBuilder::new(["foo\nbar"]);
    let re = builder.dot_matches_new_line(true).build().unwrap();
    let hay = b"foo\nbar";
    re.is_match(hay);
}

#[test]
fn test_dot_matches_new_line_false_with_new_line_in_pattern() {
    let mut builder = RegexSetBuilder::new(["foo\nbar"]);
    let re = builder.dot_matches_new_line(false).build().unwrap();
    let hay = b"foo\nbar";
    re.is_match(hay);
}

