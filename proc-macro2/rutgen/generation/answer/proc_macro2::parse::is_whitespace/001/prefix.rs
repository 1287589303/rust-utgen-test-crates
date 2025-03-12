// Answer 0

#[test]
fn test_is_whitespace_space() {
    let result = is_whitespace(' ');
}

#[test]
fn test_is_whitespace_newline() {
    let result = is_whitespace('\n');
}

#[test]
fn test_is_whitespace_tab() {
    let result = is_whitespace('\t');
}

#[test]
fn test_is_whitespace_carriage_return() {
    let result = is_whitespace('\r');
}

#[test]
fn test_is_whitespace_left_to_right_mark() {
    let result = is_whitespace('\u{200e}');
}

#[test]
fn test_is_whitespace_right_to_left_mark() {
    let result = is_whitespace('\u{200f}');
}

