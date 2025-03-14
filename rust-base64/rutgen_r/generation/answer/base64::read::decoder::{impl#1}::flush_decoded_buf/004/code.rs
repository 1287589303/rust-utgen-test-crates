// Answer 0

#[derive(Default)]
struct Decoder {
    decoded_chunk_buffer: Vec<u8>,
    decoded_len: usize,
    decoded_offset: usize,
}

impl Decoder {
    const DECODED_CHUNK_SIZE: usize = 1024;

    fn flush_decoded_buf(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        debug_assert!(self.decoded_len > 0);
        debug_assert!(!buf.is_empty());

        let copy_len = std::cmp::min(self.decoded_len, buf.len());
        debug_assert!(copy_len > 0);
        debug_assert!(copy_len <= self.decoded_len);

        buf[..copy_len].copy_from_slice(
            &self.decoded_chunk_buffer[self.decoded_offset..self.decoded_offset + copy_len],
        );

        self.decoded_offset += copy_len;
        self.decoded_len -= copy_len;

        debug_assert!(self.decoded_len < Self::DECODED_CHUNK_SIZE);

        Ok(copy_len)
    }
}

#[test]
fn test_flush_decoded_buf_copy_all() {
    let mut decoder = Decoder {
        decoded_chunk_buffer: vec![1, 2, 3, 4, 5],
        decoded_len: 5,
        decoded_offset: 0,
    };
    
    let mut buf = vec![0; 5];
    let result = decoder.flush_decoded_buf(&mut buf).unwrap();
    
    assert_eq!(result, 5);
    assert_eq!(buf, [1, 2, 3, 4, 5]);
    assert_eq!(decoder.decoded_len, 0);
    assert_eq!(decoder.decoded_offset, 5);
}

#[test]
fn test_flush_decoded_buf_partial_copy() {
    let mut decoder = Decoder {
        decoded_chunk_buffer: vec![1, 2, 3, 4, 5],
        decoded_len: 5,
        decoded_offset: 0,
    };
    
    let mut buf = vec![0; 3];
    let result = decoder.flush_decoded_buf(&mut buf).unwrap();
    
    assert_eq!(result, 3);
    assert_eq!(buf, [1, 2, 3]);
    assert_eq!(decoder.decoded_len, 2);
    assert_eq!(decoder.decoded_offset, 3);
}

#[test]
fn test_flush_decoded_buf_empty_output() {
    let mut decoder = Decoder {
        decoded_chunk_buffer: vec![1, 2],
        decoded_len: 2,
        decoded_offset: 0,
    };
    
    let mut buf = vec![]; // buf is empty
    let result = decoder.flush_decoded_buf(&mut buf).unwrap_err();
    
    // Check for error type
    assert_eq!(result.kind(), std::io::ErrorKind::Other); // Adjust as necessary for your error handling
}

