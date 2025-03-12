// Answer 0

#[test]
fn test_set_line_terminator_zero() {
    let mut matcher = LookMatcher::new();
    matcher.set_line_terminator(0);
}

#[test]
fn test_set_line_terminator_max() {
    let mut matcher = LookMatcher::new();
    matcher.set_line_terminator(255);
}

#[test]
fn test_set_line_terminator_mid() {
    let mut matcher = LookMatcher::new();
    matcher.set_line_terminator(128);
}

#[test]
fn test_set_line_terminator_small() {
    let mut matcher = LookMatcher::new();
    matcher.set_line_terminator(1);
}

#[test]
fn test_set_line_terminator_large() {
    let mut matcher = LookMatcher::new();
    matcher.set_line_terminator(254);
}

