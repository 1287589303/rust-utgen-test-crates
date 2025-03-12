// Answer 0

#[test]
fn test_fmt_start_whitespace() {
    let unicode_range = ClassUnicodeRange {
        start: ' ',
        end: ' ',
    };
    let mut buffer = core::fmt::Formatter::new();
    let _ = unicode_range.fmt(&mut buffer);
}

#[test]
fn test_fmt_start_whitespace_end_whitespace() {
    let unicode_range = ClassUnicodeRange {
        start: '\t',
        end: '\t',
    };
    let mut buffer = core::fmt::Formatter::new();
    let _ = unicode_range.fmt(&mut buffer);
}

#[test]
fn test_fmt_start_whitespace_end_newline() {
    let unicode_range = ClassUnicodeRange {
        start: ' ',
        end: '\n',
    };
    let mut buffer = core::fmt::Formatter::new();
    let _ = unicode_range.fmt(&mut buffer);
}

#[test]
fn test_fmt_start_newline_end_whitespace() {
    let unicode_range = ClassUnicodeRange {
        start: '\n',
        end: ' ',
    };
    let mut buffer = core::fmt::Formatter::new();
    let _ = unicode_range.fmt(&mut buffer);
}

#[test]
fn test_fmt_start_control_character_end_whitespace() {
    let unicode_range = ClassUnicodeRange {
        start: '\x00',
        end: ' ',
    };
    let mut buffer = core::fmt::Formatter::new();
    let _ = unicode_range.fmt(&mut buffer);
}

#[test]
fn test_fmt_start_whitespace_end_control_character() {
    let unicode_range = ClassUnicodeRange {
        start: ' ',
        end: '\x00',
    };
    let mut buffer = core::fmt::Formatter::new();
    let _ = unicode_range.fmt(&mut buffer);
}

