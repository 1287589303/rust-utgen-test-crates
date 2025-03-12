// Answer 0

#[test]
fn test_look_matcher_new_default() {
    let matcher = LookMatcher::new();
}

#[test]
fn test_look_matcher_line_terminator() {
    let matcher = LookMatcher::new();
    let line_terminator = matcher.lineterm.0;
}

