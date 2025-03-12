// Answer 0

#[test]
fn test_fmt_char_null() {
    let c = '\0';
    let unexpected = Unexpected::Char(c);
    let mut formatter = std::fmt::Formatter::default();
    let _ = unexpected.fmt(&mut formatter);
}

#[test]
fn test_fmt_char_ascii() {
    let c = 'A';
    let unexpected = Unexpected::Char(c);
    let mut formatter = std::fmt::Formatter::default();
    let _ = unexpected.fmt(&mut formatter);
}

#[test]
fn test_fmt_char_unicode() {
    let c = '你'; // A Unicode character
    let unexpected = Unexpected::Char(c);
    let mut formatter = std::fmt::Formatter::default();
    let _ = unexpected.fmt(&mut formatter);
}

#[test]
fn test_fmt_char_special() {
    let c = '#'; // A special character
    let unexpected = Unexpected::Char(c);
    let mut formatter = std::fmt::Formatter::default();
    let _ = unexpected.fmt(&mut formatter);
}

