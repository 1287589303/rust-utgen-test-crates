// Answer 0

#[test]
fn test_write_char_escape_quote() {
    struct TestFormatter;

    impl Formatter for TestFormatter {}

    let mut writer = Vec::new();
    let mut formatter = TestFormatter;

    let _ = formatter.write_char_escape(&mut writer, CharEscape::Quote);
}

#[test]
fn test_write_char_escape_reverse_solidus() {
    struct TestFormatter;

    impl Formatter for TestFormatter {}

    let mut writer = Vec::new();
    let mut formatter = TestFormatter;

    let _ = formatter.write_char_escape(&mut writer, CharEscape::ReverseSolidus);
}

#[test]
fn test_write_char_escape_solidus() {
    struct TestFormatter;

    impl Formatter for TestFormatter {}

    let mut writer = Vec::new();
    let mut formatter = TestFormatter;

    let _ = formatter.write_char_escape(&mut writer, CharEscape::Solidus);
}

#[test]
fn test_write_char_escape_backspace() {
    struct TestFormatter;

    impl Formatter for TestFormatter {}

    let mut writer = Vec::new();
    let mut formatter = TestFormatter;

    let _ = formatter.write_char_escape(&mut writer, CharEscape::Backspace);
}

#[test]
fn test_write_char_escape_form_feed() {
    struct TestFormatter;

    impl Formatter for TestFormatter {}

    let mut writer = Vec::new();
    let mut formatter = TestFormatter;

    let _ = formatter.write_char_escape(&mut writer, CharEscape::FormFeed);
}

#[test]
fn test_write_char_escape_line_feed() {
    struct TestFormatter;

    impl Formatter for TestFormatter {}

    let mut writer = Vec::new();
    let mut formatter = TestFormatter;

    let _ = formatter.write_char_escape(&mut writer, CharEscape::LineFeed);
}

#[test]
fn test_write_char_escape_carriage_return() {
    struct TestFormatter;

    impl Formatter for TestFormatter {}

    let mut writer = Vec::new();
    let mut formatter = TestFormatter;

    let _ = formatter.write_char_escape(&mut writer, CharEscape::CarriageReturn);
}

#[test]
fn test_write_char_escape_tab() {
    struct TestFormatter;

    impl Formatter for TestFormatter {}

    let mut writer = Vec::new();
    let mut formatter = TestFormatter;

    let _ = formatter.write_char_escape(&mut writer, CharEscape::Tab);
}

#[test]
fn test_write_char_escape_ascii_control() {
    struct TestFormatter;

    impl Formatter for TestFormatter {}

    let mut writer = Vec::new();
    let mut formatter = TestFormatter;

    for byte in 0..=255 {
        let _ = formatter.write_char_escape(&mut writer, CharEscape::AsciiControl(byte));
    }
}

