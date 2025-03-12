// Answer 0

#[test]
fn test_line_terminator_valid() {
    let re = RegexBuilder::new(r"^foo$")
        .multi_line(true)
        .line_terminator(b'\n')
        .build()
        .unwrap();
}

#[test]
fn test_line_terminator_valid_crlf() {
    let re = RegexBuilder::new(r"^foo$")
        .multi_line(true)
        .line_terminator(b'\r')
        .build()
        .unwrap();
}

#[test]
fn test_line_terminator_null() {
    let re = RegexBuilder::new(r"^foo$")
        .multi_line(true)
        .line_terminator(0)
        .build()
        .unwrap();
}

#[test]
fn test_line_terminator_non_ascii_fail() {
    assert!(RegexBuilder::new(r".").line_terminator(0x80).build().is_err());
}

#[test]
fn test_line_terminator_non_ascii_success() {
    let re = RegexBuilder::new(r"a")
        .line_terminator(0x80)
        .build()
        .unwrap();
}

#[test]
fn test_line_terminator_dot_match_newline() {
    let re = RegexBuilder::new(r".")
        .line_terminator(b'\x00')
        .build()
        .unwrap();
    assert!(re.is_match("\n"));
    assert!(!re.is_match("\x00"));
}

