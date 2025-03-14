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

#[test]
fn test_fmt_decoder_reader() {
    let decoder = DecoderReader {
        b64_offset: 10,
        b64_len: 20,
        decoded_chunk_buffer: vec![1, 2, 3],
        decoded_offset: 5,
        decoded_len: 3,
        input_consumed_len: 15,
        padding_offset: 0,
    };
    
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", decoder);
    
    assert!(result.is_ok());
    assert!(output.contains("b64_offset: 10"));
    assert!(output.contains("b64_len: 20"));
    assert!(output.contains("decoded_chunk_buffer: [1, 2, 3]"));
    assert!(output.contains("decoded_offset: 5"));
    assert!(output.contains("decoded_len: 3"));
    assert!(output.contains("input_consumed_len: 15"));
    assert!(output.contains("padding_offset: 0"));
}

