// Answer 0

#[test]
fn test_write_encoded_bytes_success() {
    let mut output = String::new();
    let mut sink = StringSink { string: &mut output };

    let data = b"Hello, World!";
    let result = sink.write_encoded_bytes(data);

    assert!(result.is_ok());
    assert_eq!(output, "Hello, World!");
}

#[test]
#[should_panic]
fn test_write_encoded_bytes_invalid_utf8() {
    let mut output = String::new();
    let mut sink = StringSink { string: &mut output };

    let data = &[0xFF, 0xFF, 0xFF]; // Invalid UTF-8 sequence
    let _ = sink.write_encoded_bytes(data); // This should panic
}

