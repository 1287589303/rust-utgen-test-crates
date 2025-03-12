// Answer 0

#[test]
fn test_decode_with_valid_base64_and_fragment() {
    struct TestWriter {
        data: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { data: Vec::new() }
        }

        fn write(&mut self, bytes: &[u8]) -> Result<(), ()> {
            self.data.extend_from_slice(bytes);
            Ok(())
        }
    }

    let input = "data:text/plain;base64,SGVsbG8sIFdvcmxkIQ==#fragment";
    let data_url = DataUrl::process(input).unwrap();
    let mut writer = TestWriter::new();

    data_url.decode(|bytes| writer.write(bytes)).unwrap();
}

#[test]
fn test_decode_with_valid_base64_without_fragment() {
    struct TestWriter {
        data: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { data: Vec::new() }
        }

        fn write(&mut self, bytes: &[u8]) -> Result<(), ()> {
            self.data.extend_from_slice(bytes);
            Ok(())
        }
    }

    let input = "data:text/plain;base64,SGVsbG8sIFdvcmxkIQ==";
    let data_url = DataUrl::process(input).unwrap();
    let mut writer = TestWriter::new();

    data_url.decode(|bytes| writer.write(bytes)).unwrap();
}

#[test]
fn test_decode_with_valid_base64_and_empty_fragment() {
    struct TestWriter {
        data: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { data: Vec::new() }
        }

        fn write(&mut self, bytes: &[u8]) -> Result<(), ()> {
            self.data.extend_from_slice(bytes);
            Ok(())
        }
    }

    let input = "data:text/plain;base64,SGVsbG8sIFdvcmxkIQ==#";
    let data_url = DataUrl::process(input).unwrap();
    let mut writer = TestWriter::new();

    data_url.decode(|bytes| writer.write(bytes)).unwrap();
}

