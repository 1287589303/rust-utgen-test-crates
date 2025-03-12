// Answer 0

#[test]
fn test_write_byte_array_empty() {
    struct TestFormatter;

    impl Formatter for TestFormatter {}

    let mut writer = Vec::new();
    let formatter = TestFormatter;

    formatter.write_byte_array(&mut writer, &[]).unwrap();
}

#[test]
fn test_write_byte_array_single_byte() {
    struct TestFormatter;

    impl Formatter for TestFormatter {}

    let mut writer = Vec::new();
    let formatter = TestFormatter;

    formatter.write_byte_array(&mut writer, &[0]).unwrap();
    formatter.write_byte_array(&mut writer, &[255]).unwrap();
}

#[test]
fn test_write_byte_array_multiple_bytes() {
    struct TestFormatter;

    impl Formatter for TestFormatter {}

    let mut writer = Vec::new();
    let formatter = TestFormatter;

    formatter.write_byte_array(&mut writer, &[0, 128, 255]).unwrap();
}

#[test]
#[should_panic]
fn test_write_byte_array_invalid_byte() {
    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn write_u8<W>(&mut self, _writer: &mut W, _value: u8) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            Err(io::Error::new(io::ErrorKind::Other, "invalid"))
        }
    }

    let mut writer = Vec::new();
    let formatter = TestFormatter;

    formatter.write_byte_array(&mut writer, &[55]).unwrap();
}

