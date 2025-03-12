// Answer 0

#[test]
fn test_empty_string() {
    let encoded_body_plus_fragment = "";
    let mut write_bytes_called = false;
    let write_bytes = |bytes: &[u8]| {
        write_bytes_called = true;
        Ok(())
    };
    
    let result = decode_without_base64(encoded_body_plus_fragment, write_bytes);
    assert!(result.is_ok() && result.unwrap().is_none());
}

#[test]
fn test_no_special_bytes() {
    let encoded_body_plus_fragment = "HelloWorld123";
    let mut write_bytes_called = false;
    let write_bytes = |bytes: &[u8]| {
        write_bytes_called = true;
        Ok(())
    };
    
    let result = decode_without_base64(encoded_body_plus_fragment, write_bytes);
    assert!(result.is_ok() && result.unwrap().is_none());
}

#[test]
fn test_empty_after_special_byte() {
    let encoded_body_plus_fragment = "Hello%20World#";
    let mut write_bytes_called = false;
    let write_bytes = |bytes: &[u8]| {
        write_bytes_called = true;
        Ok(())
    };

    let result = decode_without_base64(encoded_body_plus_fragment, write_bytes);
    assert!(result.is_ok() && result.unwrap().is_none());
}

