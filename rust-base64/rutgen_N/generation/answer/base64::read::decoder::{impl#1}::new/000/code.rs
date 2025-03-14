// Answer 0

#[test]
fn test_decoder_new() {
    struct DummyReader {
        data: Vec<u8>,
        position: usize,
    }

    impl DummyReader {
        fn new(data: Vec<u8>) -> Self {
            DummyReader { data, position: 0 }
        }
    }

    impl std::io::Read for DummyReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            if self.position >= self.data.len() {
                return Ok(0);
            }
            let bytes_read = std::cmp::min(buf.len(), self.data.len() - self.position);
            buf[..bytes_read].copy_from_slice(&self.data[self.position..self.position + bytes_read]);
            self.position += bytes_read;
            Ok(bytes_read)
        }
    }

    struct DummyEngine;

    let dummy_data = b"U29tZSB0ZXh0".to_vec(); // Base64 for "Some text"
    let dummy_reader = DummyReader::new(dummy_data);
    let engine = DummyEngine;

    let decoder = base64::DecoderReader::new(dummy_reader, &engine);

    assert_eq!(decoder.b64_offset, 0);
    assert_eq!(decoder.b64_len, 0);
    assert_eq!(decoder.decoded_offset, 0);
    assert_eq!(decoder.decoded_len, 0);
    assert_eq!(decoder.input_consumed_len, 0);
    assert_eq!(decoder.padding_offset, None);
}

