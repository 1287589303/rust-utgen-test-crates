// Answer 0

#[test]
fn test_decoder_new_with_empty_reader() {
    struct MockReader {
        data: Vec<u8>,
    }
    
    impl std::io::Read for MockReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let len = std::cmp::min(buf.len(), self.data.len());
            buf[..len].copy_from_slice(&self.data[..len]);
            self.data.drain(..len);
            Ok(len)
        }
    }

    struct MockEngine;

    const BUF_SIZE: usize = 64;
    const DECODED_CHUNK_SIZE: usize = 32;

    let empty_reader = MockReader { data: Vec::new() };
    let engine = &MockEngine;

    let decoder = base64::new(empty_reader, engine);

    assert_eq!(decoder.b64_buffer, [0; BUF_SIZE]);
    assert_eq!(decoder.b64_offset, 0);
    assert_eq!(decoder.b64_len, 0);
    assert_eq!(decoder.decoded_chunk_buffer, [0; DECODED_CHUNK_SIZE]);
    assert_eq!(decoder.decoded_offset, 0);
    assert_eq!(decoder.decoded_len, 0);
    assert_eq!(decoder.input_consumed_len, 0);
    assert_eq!(decoder.padding_offset, None);
}

#[test]
fn test_decoder_new_with_full_reader() {
    struct MockReader {
        data: Vec<u8>,
    }

    impl std::io::Read for MockReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let len = std::cmp::min(buf.len(), self.data.len());
            buf[..len].copy_from_slice(&self.data[..len]);
            self.data.drain(..len);
            Ok(len)
        }
    }

    struct MockEngine;

    const BUF_SIZE: usize = 64;
    const DECODED_CHUNK_SIZE: usize = 32;

    let full_reader = MockReader { data: vec![1, 2, 3, 4, 5] };
    let engine = &MockEngine;

    let decoder = base64::new(full_reader, engine);

    assert_eq!(decoder.b64_buffer, [0; BUF_SIZE]);
    assert_eq!(decoder.b64_offset, 0);
    assert_eq!(decoder.b64_len, 0);
    assert_eq!(decoder.decoded_chunk_buffer, [0; DECODED_CHUNK_SIZE]);
    assert_eq!(decoder.decoded_offset, 0);
    assert_eq!(decoder.decoded_len, 0);
    assert_eq!(decoder.input_consumed_len, 0);
    assert_eq!(decoder.padding_offset, None);
}

