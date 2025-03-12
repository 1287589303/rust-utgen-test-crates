// Answer 0

#[test]
fn test_end_string_with_buffered_writer() {
    struct BufferedWriter {
        buffer: Vec<u8>,
    }
    
    impl io::Write for BufferedWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = BufferedWriter { buffer: Vec::new() };
    let result = writer.end_string(&mut writer);
}

#[test]
fn test_end_string_with_unbuffered_writer() {
    struct UnbufferedWriter {
        output: Vec<u8>,
    }
    
    impl io::Write for UnbufferedWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.push(buf[0]);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = UnbufferedWriter { output: Vec::new() };
    let result = writer.end_string(&mut writer);
}

#[test]
fn test_end_string_with_empty_writer() {
    struct EmptyWriter;
    
    impl io::Write for EmptyWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = EmptyWriter;
    let result = writer.end_string(&mut writer);
}

