// Answer 0

#[test]
fn test_decode_without_base64_with_fragment() {
    let input = "hello%20world#fragment"; // `%20` is a space
    let mut output = Vec::new();
    let result = decode_without_base64(input, |bytes| {
        output.extend_from_slice(bytes);
        Ok(())
    });
    let expected_output = b"hello world";
    assert_eq!(output, expected_output);
    assert!(result.is_ok());
    if let Ok(Some(fragment)) = result {
        assert_eq!(fragment.0, "fragment");
    }
}

#[test]
fn test_decode_without_base64_with_tab() {
    let input = "hello%20world\tfoo"; // `%20` is a space
    let mut output = Vec::new();
    let result = decode_without_base64(input, |bytes| {
        output.extend_from_slice(bytes);
        Ok(())
    });
    let expected_output = b"hello world";
    assert_eq!(output, expected_output);
    assert!(result.is_ok());
}

#[test]
fn test_decode_without_base64_with_newline() {
    let input = "hello%20world\nfoo"; // `%20` is a space
    let mut output = Vec::new();
    let result = decode_without_base64(input, |bytes| {
        output.extend_from_slice(bytes);
        Ok(())
    });
    let expected_output = b"hello world";
    assert_eq!(output, expected_output);
    assert!(result.is_ok());
}

#[test]
fn test_decode_without_base64_with_carriage_return() {
    let input = "hello%20world\rfoo"; // `%20` is a space
    let mut output = Vec::new();
    let result = decode_without_base64(input, |bytes| {
        output.extend_from_slice(bytes);
        Ok(())
    });
    let expected_output = b"hello world";
    assert_eq!(output, expected_output);
    assert!(result.is_ok());
}

#[test]
fn test_decode_without_base64_with_invalid_percent() {
    let input = "hello%2zworld"; // Invalid hex after percent
    let mut output = Vec::new();
    let result = decode_without_base64(input, |bytes| {
        output.extend_from_slice(bytes);
        Ok(())
    });
    let expected_output = b"hello%2zworld";
    assert_eq!(output, expected_output);
    assert!(result.is_ok());
}

