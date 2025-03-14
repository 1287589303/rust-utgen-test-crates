// Answer 0

#[derive(Default)]
struct Decoder {
    decoded_chunk_buffer: Vec<u8>,
    decoded_offset: usize,
    decoded_len: usize,
}

const DECODED_CHUNK_SIZE: usize = 1024;

impl Decoder {
    fn new() -> Self {
        Self {
            decoded_chunk_buffer: vec![0; DECODED_CHUNK_SIZE],
            decoded_offset: 0,
            decoded_len: 0,
        }
    }
    
    fn set_decoded_data(&mut self, data: &[u8]) {
        self.decoded_chunk_buffer[..data.len()].copy_from_slice(data);
        self.decoded_len = data.len();
        self.decoded_offset = 0;
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
fn test_flush_decoded_buf_partial_copy() {
    let mut decoder = Decoder::new();
    decoder.set_decoded_data(&[1, 2, 3, 4, 5]);

    let mut target_buf = [0u8; 3];
    let result = decoder.flush_decoded_buf(&mut target_buf).unwrap();

    assert_eq!(result, 3);
    assert_eq!(&target_buf, &[1, 2, 3]);
}

#[test]
fn test_flush_decoded_buf_full_copy() {
    let mut decoder = Decoder::new();
    decoder.set_decoded_data(&[1, 2, 3, 4, 5]);

    let mut target_buf = [0u8; 5];
    let result = decoder.flush_decoded_buf(&mut target_buf).unwrap();

    assert_eq!(result, 5);
    assert_eq!(&target_buf, &[1, 2, 3, 4, 5]);
}

#[test]
fn test_flush_decoded_buf_empty_target() {
    let mut decoder = Decoder::new();
    decoder.set_decoded_data(&[1, 2, 3]);

    let mut target_buf = [];
    let result = decoder.flush_decoded_buf(&mut target_buf);
    
    assert!(result.is_err());
}

#[test]
fn test_flush_decoded_buf_no_data() {
    let mut decoder = Decoder::new();

    let mut target_buf = [0u8; 2];
    let result = decoder.flush_decoded_buf(&mut target_buf);

    assert!(result.is_err());
}

