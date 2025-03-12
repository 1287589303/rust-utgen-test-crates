// Answer 0

#[test]
fn test_class_unicode_range_fmt_with_non_whitespace_non_control_start_and_end() {
    let range = ClassUnicodeRange {
        start: 'a',
        end: 'z',
    };
    let mut formatter = core::fmt::Formatter::new();
    let _ = range.fmt(&mut formatter);
}

#[test]
fn test_class_unicode_range_fmt_with_whitespace_start_and_non_whitespace_non_control_end() {
    let range = ClassUnicodeRange {
        start: ' ',
        end: 'z',
    };
    let mut formatter = core::fmt::Formatter::new();
    let _ = range.fmt(&mut formatter);
}

#[test]
fn test_class_unicode_range_fmt_with_control_start_and_non_whitespace_non_control_end() {
    let range = ClassUnicodeRange {
        start: '\n',
        end: 'z',
    };
    let mut formatter = core::fmt::Formatter::new();
    let _ = range.fmt(&mut formatter);
}

#[test]
fn test_class_unicode_range_fmt_with_non_whitespace_non_control_start_and_whitespace_end() {
    let range = ClassUnicodeRange {
        start: 'a',
        end: ' ',
    };
    let mut formatter = core::fmt::Formatter::new();
    let _ = range.fmt(&mut formatter);
}

#[test]
fn test_class_unicode_range_fmt_with_non_whitespace_non_control_start_and_control_end() {
    let range = ClassUnicodeRange {
        start: 'a',
        end: '\n',
    };
    let mut formatter = core::fmt::Formatter::new();
    let _ = range.fmt(&mut formatter);
}

#[test]
fn test_class_unicode_range_fmt_with_control_start_and_control_end() {
    let range = ClassUnicodeRange {
        start: '\n',
        end: '\t',
    };
    let mut formatter = core::fmt::Formatter::new();
    let _ = range.fmt(&mut formatter);
}

