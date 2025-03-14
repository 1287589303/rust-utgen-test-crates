// Answer 0

#[test]
fn test_write_encoded_bytes_empty_string() {
    let mut output = String::new();
    let mut sink = StringSink { string: &mut output };
    let result = sink.write_encoded_bytes(b"");
    assert_eq!(result, Ok(()));
    assert_eq!(output, "");
}

#[test]
fn test_write_encoded_bytes_simple_string() {
    let mut output = String::new();
    let mut sink = StringSink { string: &mut output };
    let result = sink.write_encoded_bytes(b"Hello");
    assert_eq!(result, Ok(()));
    assert_eq!(output, "Hello");
}

#[test]
fn test_write_encoded_bytes_multiple_writes() {
    let mut output = String::new();
    let mut sink = StringSink { string: &mut output };
    let result1 = sink.write_encoded_bytes(b"Hello");
    let result2 = sink.write_encoded_bytes(b" ");
    let result3 = sink.write_encoded_bytes(b"World");
    assert_eq!(result1, Ok(()));
    assert_eq!(result2, Ok(()));
    assert_eq!(result3, Ok(()));
    assert_eq!(output, "Hello World");
}

#[test]
fn test_write_encoded_bytes_special_characters() {
    let mut output = String::new();
    let mut sink = StringSink { string: &mut output };
    let result = sink.write_encoded_bytes(b"Hello, 世界");
    assert_eq!(result, Ok(()));
    assert_eq!(output, "Hello, 世界");
}

