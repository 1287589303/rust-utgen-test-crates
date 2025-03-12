// Answer 0

#[test]
fn test_with_formatter_buffer_writer() {
    struct BufferWriter {
        data: Vec<u8>,
    }

    impl io::Write for BufferWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct SimpleFormatter;

    impl Formatter for SimpleFormatter {}

    let writer = BufferWriter { data: Vec::new() };
    let formatter = SimpleFormatter;
    let serializer = Serializer::with_formatter(writer, formatter);
}

#[test]
fn test_with_formatter_file_writer() {
    use std::fs::File;
    use std::io::Write;

    struct SimpleFormatter;

    impl Formatter for SimpleFormatter {}

    let file = File::create("test_file.json").unwrap();
    let formatter = SimpleFormatter;
    let serializer = Serializer::with_formatter(file, formatter);
}

#[test]
fn test_with_formatter_custom_writer() {
    struct CustomWriter {
        output: String,
    }

    impl io::Write for CustomWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.push_str(std::str::from_utf8(buf).unwrap());
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct SimpleFormatter;

    impl Formatter for SimpleFormatter {}

    let writer = CustomWriter { output: String::new() };
    let formatter = SimpleFormatter;
    let serializer = Serializer::with_formatter(writer, formatter);
}

