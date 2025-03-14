// Answer 0

#[test]
fn test_read_empty_buffer() {
    struct DummyReader;

    impl std::io::Read for DummyReader {
        fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
            Ok(0)
        }
    }

    let mut buf = [0; 10];
    let mut reader = Decoder {
        b64_offset: 0,
        b64_len: 0,
        decoded_len: 0,
        decoded_offset: 0,
        b64_buffer: [0; BUF_SIZE],
        decoded_chunk_buffer: [0; DECODED_CHUNK_SIZE],
        delegate: DummyReader,
    };
    
    let result = reader.read(&mut buf);
    assert_eq!(result, Ok(0));
}

#[test]
fn test_read_small_buffer_at_eof() {
    struct DummyReader;

    impl std::io::Read for DummyReader {
        fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
            Ok(0)
        }
    }

    let mut buf = [0; 2]; // Smaller than DECODED_CHUNK_SIZE
    let mut reader = Decoder {
        b64_offset: 0,
        b64_len: 0,
        decoded_len: 0,
        decoded_offset: 0,
        b64_buffer: [0; BUF_SIZE],
        decoded_chunk_buffer: [0; DECODED_CHUNK_SIZE],
        delegate: DummyReader,
    };

    let result = reader.read(&mut buf);
    assert_eq!(result, Ok(0));
}

#[test]
fn test_read_partial_decode() {
    struct DummyReader {
        read_count: usize,
    }

    impl std::io::Read for DummyReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            if self.read_count < 1 {
                self.read_count += 1;
                buf.copy_from_slice(b"SGVsbG8="); // Base64 for "Hello"
                Ok(8) // 8 bytes read
            } else {
                Ok(0) // EOF after first read
            }
        }
    }

    let mut buf = [0; DECODED_CHUNK_SIZE];
    let mut reader = Decoder {
        b64_offset: 0,
        b64_len: 8,
        decoded_len: 0,
        decoded_offset: 0,
        b64_buffer: [0; BUF_SIZE],
        decoded_chunk_buffer: [0; DECODED_CHUNK_SIZE],
        delegate: DummyReader { read_count: 0 },
    };

    let result = reader.read(&mut buf);
    assert_eq!(result, Ok(5)); // "Hello" has 5 bytes
    assert_eq!(&buf[..5], b"Hello");
}

