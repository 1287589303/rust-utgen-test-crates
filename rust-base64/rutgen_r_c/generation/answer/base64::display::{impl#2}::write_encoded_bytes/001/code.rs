// Answer 0

#[test]
fn test_write_encoded_bytes_valid_utf8() {
    let mut output = String::new();
    let mut formatter = FormatterSink {
        f: &mut output,
    };
    let encoded: &[u8] = b"ValidUTF8String";
    
    let result = formatter.write_encoded_bytes(encoded);
    
    assert!(result.is_ok());
    assert_eq!(output, "ValidUTF8String");
}

#[test]
#[should_panic(expected = "base64 data was not utf8")]
fn test_write_encoded_bytes_invalid_utf8() {
    let mut output = String::new();
    let mut formatter = FormatterSink {
        f: &mut output,
    };
    let encoded: &[u8] = &[0xFF, 0xFE, 0xFD]; // Invalid UTF-8 bytes
    
    let _ = formatter.write_encoded_bytes(encoded);
}

#[test]
fn test_write_encoded_bytes_empty() {
    let mut output = String::new();
    let mut formatter = FormatterSink {
        f: &mut output,
    };
    let encoded: &[u8] = b""; // Empty byte slice
    
    let result = formatter.write_encoded_bytes(encoded);
    
    assert!(result.is_ok());
    assert_eq!(output, ""); // Output should remain empty
}

