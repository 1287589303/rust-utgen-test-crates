// Answer 0

#[test]
fn test_write_char_escape_quote() {
    struct TestFormatter;

    let mut writer = Vec::new();
    let mut formatter = TestFormatter;
    formatter.write_char_escape(&mut writer, CharEscape::Quote).unwrap();
}

#[test]
fn test_write_char_escape_reverse_solidus() {
    struct TestFormatter;

    let mut writer = Vec::new();
    let mut formatter = TestFormatter;
    formatter.write_char_escape(&mut writer, CharEscape::ReverseSolidus).unwrap();
}

#[test]
fn test_write_char_escape_solidus() {
    struct TestFormatter;

    let mut writer = Vec::new();
    let mut formatter = TestFormatter;
    formatter.write_char_escape(&mut writer, CharEscape::Solidus).unwrap();
}

#[test]
fn test_write_char_escape_backspace() {
    struct TestFormatter;

    let mut writer = Vec::new();
    let mut formatter = TestFormatter;
    formatter.write_char_escape(&mut writer, CharEscape::Backspace).unwrap();
}

#[test]
fn test_write_char_escape_form_feed() {
    struct TestFormatter;

    let mut writer = Vec::new();
    let mut formatter = TestFormatter;
    formatter.write_char_escape(&mut writer, CharEscape::FormFeed).unwrap();
}

#[test]
fn test_write_char_escape_line_feed() {
    struct TestFormatter;

    let mut writer = Vec::new();
    let mut formatter = TestFormatter;
    formatter.write_char_escape(&mut writer, CharEscape::LineFeed).unwrap();
}

#[test]
fn test_write_char_escape_carriage_return() {
    struct TestFormatter;

    let mut writer = Vec::new();
    let mut formatter = TestFormatter;
    formatter.write_char_escape(&mut writer, CharEscape::CarriageReturn).unwrap();
}

#[test]
fn test_write_char_escape_tab() {
    struct TestFormatter;

    let mut writer = Vec::new();
    let mut formatter = TestFormatter;
    formatter.write_char_escape(&mut writer, CharEscape::Tab).unwrap();
}

#[test]
fn test_write_char_escape_ascii_control() {
    struct TestFormatter;

    let mut writer = Vec::new();
    let mut formatter = TestFormatter;
    for byte in 0..=255 {
        formatter.write_char_escape(&mut writer, CharEscape::AsciiControl(byte)).unwrap();
    }
}

