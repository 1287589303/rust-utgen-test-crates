// Answer 0

#[derive(Debug)]
struct DecoderReader {
    b64_offset: usize,
    b64_len: usize,
    decoded_chunk_buffer: Vec<u8>,
    decoded_offset: usize,
    decoded_len: usize,
    input_consumed_len: usize,
    padding_offset: usize,
}

impl DecoderReader {
    fn new(
        b64_offset: usize,
        b64_len: usize,
        decoded_chunk_buffer: Vec<u8>,
        decoded_offset: usize,
        decoded_len: usize,
        input_consumed_len: usize,
        padding_offset: usize,
    ) -> Self {
        Self {
            b64_offset,
            b64_len,
            decoded_chunk_buffer,
            decoded_offset,
            decoded_len,
            input_consumed_len,
            padding_offset,
        }
    }
}

#[test]
fn test_fmt_empty_buffer() {
    let decoder = DecoderReader::new(0, 0, Vec::new(), 0, 0, 0, 0);
    let result = format!("{:?}", decoder);
    assert!(result.contains("b64_offset: 0"));
    assert!(result.contains("b64_len: 0"));
    assert!(result.contains("decoded_chunk_buffer: []"));
    assert!(result.contains("decoded_offset: 0"));
    assert!(result.contains("decoded_len: 0"));
    assert!(result.contains("input_consumed_len: 0"));
    assert!(result.contains("padding_offset: 0"));
}

#[test]
fn test_fmt_partial_data() {
    let buffer = vec![1, 2, 3];
    let decoder = DecoderReader::new(10, 5, buffer.clone(), 1, 3, 2, 0);
    let result = format!("{:?}", decoder);
    assert!(result.contains("b64_offset: 10"));
    assert!(result.contains("b64_len: 5"));
    assert!(result.contains("decoded_chunk_buffer: [1, 2, 3]"));
    assert!(result.contains("decoded_offset: 1"));
    assert!(result.contains("decoded_len: 3"));
    assert!(result.contains("input_consumed_len: 2"));
    assert!(result.contains("padding_offset: 0"));
}

#[test]
fn test_fmt_full_data() {
    let buffer = vec![255, 254, 253];
    let decoder = DecoderReader::new(20, 10, buffer.clone(), 2, 10, 10, 1);
    let result = format!("{:?}", decoder);
    assert!(result.contains("b64_offset: 20"));
    assert!(result.contains("b64_len: 10"));
    assert!(result.contains("decoded_chunk_buffer: [255, 254, 253]"));
    assert!(result.contains("decoded_offset: 2"));
    assert!(result.contains("decoded_len: 10"));
    assert!(result.contains("input_consumed_len: 10"));
    assert!(result.contains("padding_offset: 1"));
}

