// Answer 0

#[test]
fn test_line_terminator_zero() {
    let _ = RegexSetBuilder::new([r"^test$"])
        .line_terminator(0)
        .build()
        .unwrap();
}

#[test]
fn test_line_terminator_max_byte() {
    let _ = RegexSetBuilder::new([r"^test$"])
        .line_terminator(255)
        .build()
        .unwrap();
}

#[test]
fn test_line_terminator_unicode_false() {
    let _ = RegexSetBuilder::new([r"^test$"])
        .unicode(false)
        .line_terminator(127)
        .build()
        .unwrap();
}

#[test]
fn test_line_terminator_multi_line() {
    let _ = RegexSetBuilder::new([r"^test$"])
        .multi_line(true)
        .line_terminator(10)
        .build()
        .unwrap();
}

#[test]
fn test_line_terminator_case_insensitive() {
    let _ = RegexSetBuilder::new([r"^test$"])
        .case_insensitive(true)
        .line_terminator(10)
        .build()
        .unwrap();
}

#[test]
fn test_line_terminator_dot_matches_new_line() {
    let _ = RegexSetBuilder::new([r"."])
        .dot_matches_new_line(true)
        .line_terminator(10)
        .build()
        .unwrap();
}

#[test]
fn test_line_terminator_crlf_mode() {
    let _ = RegexSetBuilder::new([r"^test$"])
        .crlf(true)
        .line_terminator(10)
        .build()
        .unwrap();
}

