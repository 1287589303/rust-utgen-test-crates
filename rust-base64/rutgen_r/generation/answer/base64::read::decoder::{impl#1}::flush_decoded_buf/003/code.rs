// Answer 0

#[derive(Debug)]
struct Decoder {
    decoded_chunk_buffer: Vec<u8>,
    decoded_len: usize,
    decoded_offset: usize,
}

const DECODED_CHUNK_SIZE: usize = 16;

impl Decoder {
    fn new(buffer: Vec<u8>, length: usize) -> Self {
        Self {
            decoded_chunk_buffer: buffer,
            decoded_len: length,
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
fn test_flush_decoded_buf_full_copy() {
    let mut decoder = Decoder::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], 16);
    let mut buf = [0u8; 16];
    
    let result = decoder.flush_decoded_buf(&mut buf);
    
    assert_eq!(result.unwrap(), 16);
    assert_eq!(buf, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    assert_eq!(decoder.decoded_len, 0);
    assert_eq!(decoder.decoded_offset, 16);
}

#[test]
fn test_flush_decoded_buf_partial_copy() {
    let mut decoder = Decoder::new(vec![1, 2, 3, 4, 5, 6, 7, 8], 8);
    let mut buf = [0u8; 10];
    
    let result = decoder.flush_decoded_buf(&mut buf);
    
    assert_eq!(result.unwrap(), 8);
    assert_eq!(&buf[..8], &[1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(decoder.decoded_len, 0);
    assert_eq!(decoder.decoded_offset, 8);
}

#[test]
#[should_panic]
fn test_flush_decoded_buf_empty_buffer() {
    let mut decoder = Decoder::new(vec![1, 2, 3, 4], 4);
    let mut buf: [u8; 0] = [];
    
    decoder.flush_decoded_buf(&mut buf).unwrap();
}

#[test]
#[should_panic]
fn test_flush_decoded_buf_zero_length() {
    let mut decoder = Decoder::new(vec![1, 2, 3, 4], 4);
    let mut buf = [0u8; 0];
    
    decoder.decoded_len = 0; // Set to zero to trigger the assertion.
    decoder.flush_decoded_buf(&mut buf).unwrap();
}

