// Answer 0

#[test]
fn test_get_line_terminator_default() {
    let matcher = LookMatcher::new();
    matcher.get_line_terminator();
}

#[test]
fn test_get_line_terminator_set_to_newline() {
    let mut matcher = LookMatcher::new();
    matcher.set_line_terminator(b'\n');
    matcher.get_line_terminator();
}

#[test]
fn test_get_line_terminator_set_to_carriage_return() {
    let mut matcher = LookMatcher::new();
    matcher.set_line_terminator(b'\r');
    matcher.get_line_terminator();
}

#[test]
fn test_get_line_terminator_set_to_value_0() {
    let mut matcher = LookMatcher::new();
    matcher.set_line_terminator(0);
    matcher.get_line_terminator();
}

#[test]
fn test_get_line_terminator_set_to_value_255() {
    let mut matcher = LookMatcher::new();
    matcher.set_line_terminator(255);
    matcher.get_line_terminator();
}

#[test]
fn test_get_line_terminator_set_to_value_100() {
    let mut matcher = LookMatcher::new();
    matcher.set_line_terminator(100);
    matcher.get_line_terminator();
}

