// Answer 0

#[test]
fn test_fmt_unsupported_look_word_ascii() {
    let kind = BuildErrorKind::UnsupportedLook { look: Look::WordAscii };
    let error = BuildError { kind };
    let mut formatter = core::fmt::Formatter::new();
    let _ = error.fmt(&mut formatter);
}

#[test]
fn test_fmt_unsupported_look_word_ascii_negate() {
    let kind = BuildErrorKind::UnsupportedLook { look: Look::WordAsciiNegate };
    let error = BuildError { kind };
    let mut formatter = core::fmt::Formatter::new();
    let _ = error.fmt(&mut formatter);
}

#[test]
fn test_fmt_unsupported_look_word_unicode() {
    let kind = BuildErrorKind::UnsupportedLook { look: Look::WordUnicode };
    let error = BuildError { kind };
    let mut formatter = core::fmt::Formatter::new();
    let _ = error.fmt(&mut formatter);
}

#[test]
fn test_fmt_unsupported_look_word_unicode_negate() {
    let kind = BuildErrorKind::UnsupportedLook { look: Look::WordUnicodeNegate };
    let error = BuildError { kind };
    let mut formatter = core::fmt::Formatter::new();
    let _ = error.fmt(&mut formatter);
}

#[test]
fn test_fmt_unsupported_look_word_start_ascii() {
    let kind = BuildErrorKind::UnsupportedLook { look: Look::WordStartAscii };
    let error = BuildError { kind };
    let mut formatter = core::fmt::Formatter::new();
    let _ = error.fmt(&mut formatter);
}

#[test]
fn test_fmt_unsupported_look_word_end_ascii() {
    let kind = BuildErrorKind::UnsupportedLook { look: Look::WordEndAscii };
    let error = BuildError { kind };
    let mut formatter = core::fmt::Formatter::new();
    let _ = error.fmt(&mut formatter);
}

#[test]
fn test_fmt_unsupported_look_word_start_unicode() {
    let kind = BuildErrorKind::UnsupportedLook { look: Look::WordStartUnicode };
    let error = BuildError { kind };
    let mut formatter = core::fmt::Formatter::new();
    let _ = error.fmt(&mut formatter);
}

#[test]
fn test_fmt_unsupported_look_word_end_unicode() {
    let kind = BuildErrorKind::UnsupportedLook { look: Look::WordEndUnicode };
    let error = BuildError { kind };
    let mut formatter = core::fmt::Formatter::new();
    let _ = error.fmt(&mut formatter);
}

