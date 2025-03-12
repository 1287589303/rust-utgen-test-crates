// Answer 0

#[test]
fn test_line_terminator_zero() {
    let mut builder = RegexBuilder::new(r".");
    builder.line_terminator(0);
}

#[test]
fn test_line_terminator_ascii() {
    let mut builder = RegexBuilder::new(r".");
    builder.line_terminator(b'\n');
}

#[test]
fn test_line_terminator_crlf() {
    let mut builder = RegexBuilder::new(r".");
    builder.line_terminator(b'\r');
}

#[test]
fn test_line_terminator_special_character() {
    let mut builder = RegexBuilder::new(r".");
    builder.line_terminator(b'\x00');
}

#[test]
fn test_line_terminator_non_ascii() {
    let mut builder = RegexBuilder::new(r".");
    builder.line_terminator(0x80);
}

#[test]
fn test_line_terminator_boundary_case_max() {
    let mut builder = RegexBuilder::new(r".");
    builder.line_terminator(255);
}

