// Answer 0

#[test]
fn test_write_success() {
    struct TestEncoder {
        data: Vec<u8>,
    }

    impl TestEncoder {
        fn new() -> Self {
            TestEncoder { data: Vec::new() }
        }

        fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }
    }

    struct EncoderStringWriter {
        encoder: TestEncoder,
    }

    impl EncoderStringWriter {
        fn new() -> Self {
            EncoderStringWriter { encoder: TestEncoder::new() }
        }

        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.encoder.write(buf)
        }
    }

    let mut writer = EncoderStringWriter::new();
    let input_data = b"Test data";
    
    let result = writer.write(input_data).unwrap();
    assert_eq!(result, input_data.len());
}

#[test]
fn test_write_empty_buffer() {
    struct TestEncoder {
        data: Vec<u8>,
    }

    impl TestEncoder {
        fn new() -> Self {
            TestEncoder { data: Vec::new() }
        }

        fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }
    }

    struct EncoderStringWriter {
        encoder: TestEncoder,
    }

    impl EncoderStringWriter {
        fn new() -> Self {
            EncoderStringWriter { encoder: TestEncoder::new() }
        }

        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.encoder.write(buf)
        }
    }

    let mut writer = EncoderStringWriter::new();
    let input_data: &[u8] = &[];
    
    let result = writer.write(input_data).unwrap();
    assert_eq!(result, 0);
}

