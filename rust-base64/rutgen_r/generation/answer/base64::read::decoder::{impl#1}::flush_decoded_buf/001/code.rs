// Answer 0

#[test]
fn test_flush_decoded_buf_non_empty_buf() {
    use std::io;

    struct Decoder {
        decoded_length: usize,
        decoded_offset: usize,
        decoded_chunk_buffer: Vec<u8>,
    }

    const DECODED_CHUNK_SIZE: usize = 1024;

    impl Decoder {
        fn new() -> Self {
            Self {
                decoded_length: 10,
                decoded_offset: 0,
                decoded_chunk_buffer: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            }
        }

        fn flush_decoded_buf(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            debug_assert!(self.decoded_length > 0);
            debug_assert!(!buf.is_empty());

            let copy_len = std::cmp::min(self.decoded_length, buf.len());
            debug_assert!(copy_len > 0);
            debug_assert!(copy_len <= self.decoded_length);

            buf[..copy_len].copy_from_slice(
                &self.decoded_chunk_buffer[self.decoded_offset..self.decoded_offset + copy_len],
            );

            self.decoded_offset += copy_len;
            self.decoded_length -= copy_len;

            debug_assert!(self.decoded_length < DECODED_CHUNK_SIZE);
            Ok(copy_len)
        }
    }

    let mut decoder = Decoder::new();
    let mut buffer = vec![0; 5]; // Buffer initialized with non-empty size
    let result = decoder.flush_decoded_buf(&mut buffer);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 5);
    assert_eq!(buffer, vec![1, 2, 3, 4, 5]);
}

#[test]
#[should_panic]
fn test_flush_decoded_buf_empty_buf() {
    use std::io;

    struct Decoder {
        decoded_length: usize,
        decoded_offset: usize,
        decoded_chunk_buffer: Vec<u8>,
    }

    const DECODED_CHUNK_SIZE: usize = 1024;

    impl Decoder {
        fn new() -> Self {
            Self {
                decoded_length: 10,
                decoded_offset: 0,
                decoded_chunk_buffer: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            }
        }

        fn flush_decoded_buf(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            debug_assert!(self.decoded_length > 0);
            debug_assert!(!buf.is_empty());

            let copy_len = std::cmp::min(self.decoded_length, buf.len());
            debug_assert!(copy_len > 0);
            debug_assert!(copy_len <= self.decoded_length);

            buf[..copy_len].copy_from_slice(
                &self.decoded_chunk_buffer[self.decoded_offset..self.decoded_offset + copy_len],
            );

            self.decoded_offset += copy_len;
            self.decoded_length -= copy_len;

            debug_assert!(self.decoded_length < DECODED_CHUNK_SIZE);
            Ok(copy_len)
        }
    }

    let mut decoder = Decoder::new();
    let mut buffer: Vec<u8> = vec![]; // Buffer is empty
    let _ = decoder.flush_decoded_buf(&mut buffer);
}

