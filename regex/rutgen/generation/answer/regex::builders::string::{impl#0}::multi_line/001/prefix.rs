// Answer 0

#[test]
fn test_multi_line_true() {
    let re = RegexBuilder::new(r"^foo$")
        .multi_line(true)
        .build()
        .unwrap();
}

#[test]
fn test_multi_line_false() {
    let re = RegexBuilder::new(r"^foo$")
        .multi_line(false)
        .build()
        .unwrap();
}

#[test]
fn test_multi_line_with_line_terminator() {
    let re = RegexBuilder::new(r"^foo$")
        .line_terminator(b'\n')
        .multi_line(true)
        .build()
        .unwrap();
}

#[test]
fn test_multi_line_with_crlf() {
    let re = RegexBuilder::new(r"^foo$")
        .crlf(true)
        .multi_line(true)
        .build()
        .unwrap();
}

#[test]
fn test_multi_line_empty_pattern() {
    let re = RegexBuilder::new(r"")
        .multi_line(true)
        .build()
        .unwrap();
}

