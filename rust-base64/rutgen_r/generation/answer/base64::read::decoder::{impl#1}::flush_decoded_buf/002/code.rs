// Answer 0

#[derive(Debug)]
struct Decoder {
    decoded_chunk_buffer: Vec<u8>,
    decoded_len: usize,
    decoded_offset: usize,
}

const DECODED_CHUNK_SIZE: usize = 1024;

impl Decoder {
    fn new(buffer: Vec<u8>, len: usize) -> Self {
        Decoder {
            decoded_chunk_buffer: buffer,
            decoded_len: len,
            decoded_offset: 0,
        }
    }

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

        debug_assert!(self.decoded_len < DECODED_CHUNK_SIZE);

        Ok(copy_len)
    }
}

#[test]
fn test_flush_decoded_buf_fully_copies_data() {
    let mut decoder = Decoder::new(vec![1, 2, 3, 4, 5], 5);
    let mut buffer = [0u8; 5];
    
    let result = decoder.flush_decoded_buf(&mut buffer);
    
    assert_eq!(result, Ok(5));
    assert_eq!(buffer, [1, 2, 3, 4, 5]);
    assert_eq!(decoder.decoded_len, 0);
}

#[test]
fn test_flush_decoded_buf_partial_copies_data() {
    let mut decoder = Decoder::new(vec![1, 2, 3, 4, 5], 5);
    let mut buffer = [0u8; 3];
    
    let result = decoder.flush_decoded_buf(&mut buffer);
    
    assert_eq!(result, Ok(3));
    assert_eq!(buffer, [1, 2, 3]);
    assert_eq!(decoder.decoded_len, 2);
}

#[test]
fn test_flush_decoded_buf_exact_fit() {
    let mut decoder = Decoder::new(vec![1, 2], 2);
    let mut buffer = [0u8; 2];
    
    let result = decoder.flush_decoded_buf(&mut buffer);
    
    assert_eq!(result, Ok(2));
    assert_eq!(buffer, [1, 2]);
    assert_eq!(decoder.decoded_len, 0);
}

#[test]
fn test_flush_decoded_buf_with_reduced_length() {
    let mut decoder = Decoder::new(vec![1, 2, 3, 4], 4);
    let mut buffer = [0u8; 4];
    
    let _ = decoder.flush_decoded_buf(&mut buffer);
    
    let next_result = decoder.flush_decoded_buf(&mut buffer);
    
    assert_eq!(next_result, Ok(0));
    assert_eq!(buffer, [1, 2, 3, 4]);
    assert_eq!(decoder.decoded_len, 0);
}

#[test]
fn test_flush_decoded_buf_empty_buffer() {
    let mut decoder = Decoder::new(vec![1, 2, 3], 3);
    let mut buffer = [0u8; 0]; // empty buffer
    
    let result = decoder.flush_decoded_buf(&mut buffer);
    
    assert!(result.is_err());
}

