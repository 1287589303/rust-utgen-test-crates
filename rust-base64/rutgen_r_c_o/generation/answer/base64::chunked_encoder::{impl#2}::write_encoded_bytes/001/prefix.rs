// Answer 0

#[test]
fn test_write_encoded_bytes_single_byte_utf8() {
    let mut output = String::new();
    let mut sink = StringSink { string: &mut output };
    let input = b"a"; // single-byte UTF-8 character
    let _ = sink.write_encoded_bytes(input);
}

#[test]
fn test_write_encoded_bytes_multi_byte_utf8() {
    let mut output = String::new();
    let mut sink = StringSink { string: &mut output };
    let input = "こんにちは".as_bytes(); // multi-byte UTF-8 characters
    let _ = sink.write_encoded_bytes(input);
}

#[test]
fn test_write_encoded_bytes_empty_array() {
    let mut output = String::new();
    let mut sink = StringSink { string: &mut output };
    let input: &[u8] = &[]; // empty byte array
    let _ = sink.write_encoded_bytes(input);
}

#[test]
#[should_panic]
fn test_write_encoded_bytes_invalid_utf8() {
    let mut output = String::new();
    let mut sink = StringSink { string: &mut output };
    let input = &[0xff, 0xfe, 0xfd]; // invalid UTF-8 sequence
    let _ = sink.write_encoded_bytes(input);
}

