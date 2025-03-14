// Answer 0

#[test]
fn test_write_all_encoded_output_error_non_interrupted() {
    struct TestEngine;

    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<(), io::Error> { Ok(()) }
        fn config(&self) -> &Self::Config { &() }
    }

    struct MockWriter {
        response: Result<usize, io::Error>
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            self.response.clone()
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let engine = TestEngine;
    let mut writer = MockWriter { response: Err(io::Error::new(ErrorKind::Other, "An error occurred")) };
    let mut encoder = EncoderWriter::new(writer, &engine);
    encoder.output_occupied_len = 5; // Simulate output occupied

    let _ = encoder.write_all_encoded_output();
}

#[test]
fn test_write_all_encoded_output_error_another_non_interrupted() {
    struct TestEngine;

    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<(), io::Error> { Ok(()) }
        fn config(&self) -> &Self::Config { &() }
    }

    struct MockWriter {
        response: Result<usize, io::Error>
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            self.response.clone()
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let engine = TestEngine;
    let mut writer = MockWriter { response: Err(io::Error::new(ErrorKind::BrokenPipe, "Pipe broken")) };
    let mut encoder = EncoderWriter::new(writer, &engine);
    encoder.output_occupied_len = 10; // Simulate output occupied

    let _ = encoder.write_all_encoded_output();
}

#[test]
fn test_write_all_encoded_output_error_with_partial_write() {
    struct TestEngine;

    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<(), io::Error> { Ok(()) }
        fn config(&self) -> &Self::Config { &() }
    }

    struct MockWriter {
        response: Result<usize, io::Error>
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            self.response.clone()
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let engine = TestEngine;
    let mut writer = MockWriter { response: Err(io::Error::new(ErrorKind::PermissionDenied, "Permission denied")) };
    let mut encoder = EncoderWriter::new(writer, &engine);
    encoder.output_occupied_len = 15; // Simulate output occupied

    let _ = encoder.write_all_encoded_output();
}

