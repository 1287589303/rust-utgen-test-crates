// Answer 0

#[test]
fn test_finish_with_valid_delegate_and_no_extra_input() {
    struct MockEngine;
    struct MockWriter {
        data: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    
    let engine = MockEngine;
    let writer = MockWriter { data: Vec::new() };
    let mut encoder_writer = EncoderWriter::new(writer, &engine);
    encoder_writer.extra_input_occupied_len = 0;

    let result = encoder_writer.finish();
    // Call to finish should work without any extra input
}

#[test]
fn test_finish_with_valid_delegate_and_some_extra_input() {
    struct MockEngine;
    struct MockWriter {
        data: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    
    let engine = MockEngine;
    let writer = MockWriter { data: Vec::new() };
    let mut encoder_writer = EncoderWriter::new(writer, &engine);
    encoder_writer.extra_input_occupied_len = 2; // Filling some extra input

    let result = encoder_writer.finish();
    // Call to finish should work with extra input occupied
}

#[test]
fn test_finish_with_valid_delegate_and_full_extra_input() {
    struct MockEngine;
    struct MockWriter {
        data: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    
    let engine = MockEngine;
    let writer = MockWriter { data: Vec::new() };
    let mut encoder_writer = EncoderWriter::new(writer, &engine);
    encoder_writer.extra_input_occupied_len = 3; // Maximum extra input

    let result = encoder_writer.finish();
    // Call to finish should proceed with fully occupied extra input
}

