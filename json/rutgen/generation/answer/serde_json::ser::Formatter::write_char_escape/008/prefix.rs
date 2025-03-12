// Answer 0

#[test]
fn test_write_reverse_solidus() {
    struct TestFormatter;

    impl Formatter for TestFormatter {}

    let mut writer: Vec<u8> = Vec::new();
    let mut formatter = TestFormatter;

    formatter.write_char_escape(&mut writer, CharEscape::ReverseSolidus).unwrap();

    // Actual invocation without check
}

#[test]
fn test_write_backslash_character() {
    struct TestFormatter;

    impl Formatter for TestFormatter {}

    let mut writer: Vec<u8> = Vec::new();
    let mut formatter = TestFormatter;

    formatter.write_char_escape(&mut writer, CharEscape::ReverseSolidus).unwrap();

    // Actual invocation without check
}

