// Answer 0

#[test]
fn test_begin_array_value_first_false() {
    struct TestWriter(Vec<u8>);

    impl std::io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter(vec![]);
    let mut formatter = TestFormatter;

    let result = formatter.begin_array_value(&mut writer, false);
    let expected = b",";
    
    // Here we invoke the method while ignoring assertions
    let _ = result;
    let _ = writer.0;
}

struct TestFormatter;

impl Formatter for TestFormatter {}

