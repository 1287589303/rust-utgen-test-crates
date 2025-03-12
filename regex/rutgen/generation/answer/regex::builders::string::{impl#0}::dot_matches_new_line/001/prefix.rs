// Answer 0

#[test]
fn test_dot_matches_new_line_enabled() {
    let re = RegexBuilder::new(r"foo.bar")
        .dot_matches_new_line(true)
        .build()
        .unwrap();
    let hay = "foo\nbar";
    let _ = re.find(hay);
}

#[test]
fn test_dot_matches_new_line_disabled() {
    let re = RegexBuilder::new(r"foo.bar")
        .dot_matches_new_line(false)
        .build()
        .unwrap();
    let hay = "foo\nbar";
    let _ = re.find(hay);
}

#[test]
fn test_dot_matches_new_line_edge_case() {
    let re = RegexBuilder::new(r"foo.bar")
        .dot_matches_new_line(true)
        .build()
        .unwrap();
    let hay = "foobar";
    let _ = re.find(hay);
}

#[test]
fn test_dot_matches_new_line_with_other_flags() {
    let re = RegexBuilder::new(r"foo.bar")
        .case_insensitive(true)
        .multi_line(true)
        .dot_matches_new_line(true)
        .build()
        .unwrap();
    let hay = "FOO\nBAR";
    let _ = re.find(hay);
}

