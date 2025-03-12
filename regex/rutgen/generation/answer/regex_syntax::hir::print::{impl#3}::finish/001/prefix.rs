// Answer 0

#[test]
fn test_finish_with_string_writer() {
    struct StringWriter {
        buffer: String,
    }

    impl fmt::Write for StringWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.push_str(s);
            Ok(())
        }
    }

    let writer = Writer { wtr: StringWriter { buffer: String::new() } };
    let _result = writer.finish();
}

#[test]
fn test_finish_with_vec_u8_writer() {
    struct VecWriter {
        buffer: Vec<u8>,
    }

    impl fmt::Write for VecWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.extend_from_slice(s.as_bytes());
            Ok(())
        }
    }

    let writer = Writer { wtr: VecWriter { buffer: Vec::new() } };
    let _result = writer.finish();
}

#[test]
fn test_finish_with_fmt_buffer_writer() {
    use std::fmt::Write as FmtWrite;

    struct FmtBufferWriter {
        buffer: String,
    }

    impl FmtWrite for FmtBufferWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.push_str(s);
            Ok(())
        }
    }

    let writer = Writer { wtr: FmtBufferWriter { buffer: String::new() } };
    let _result = writer.finish();
}

