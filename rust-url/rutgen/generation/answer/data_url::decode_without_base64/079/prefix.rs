// Answer 0

#[test]
fn test_decode_without_base64_fragment() {
    let input = "abc#def";
    let mut output = Vec::new();
    let result = decode_without_base64(input, |bytes| {
        output.extend_from_slice(bytes);
        Ok(())
    });
    let fragment = FragmentIdentifier("def");
    assert!(result.is_ok() && result.unwrap() == Some(fragment));
}

#[test]
fn test_decode_without_base64_fragment_encoded_space() {
    let input = "%20#fragment";
    let mut output = Vec::new();
    let result = decode_without_base64(input, |bytes| {
        output.extend_from_slice(bytes);
        Ok(())
    });
    let fragment = FragmentIdentifier("fragment");
    assert!(result.is_ok() && result.unwrap() == Some(fragment));
}

#[test]
fn test_decode_without_base64_multiple_encoded_chars() {
    let input = "%41%42#xyz";
    let mut output = Vec::new();
    let result = decode_without_base64(input, |bytes| {
        output.extend_from_slice(bytes);
        Ok(())
    });
    let fragment = FragmentIdentifier("xyz");
    assert!(result.is_ok() && result.unwrap() == Some(fragment));
}

#[test]
fn test_decode_without_base64_special_character() {
    let input = "%23#special";
    let mut output = Vec::new();
    let result = decode_without_base64(input, |bytes| {
        output.extend_from_slice(bytes);
        Ok(())
    });
    let fragment = FragmentIdentifier("special");
    assert!(result.is_ok() && result.unwrap() == Some(fragment));
}

#[test]
fn test_decode_without_base64_empty() {
    let input = "";
    let mut output = Vec::new();
    let result = decode_without_base64(input, |bytes| {
        output.extend_from_slice(bytes);
        Ok(())
    });
    assert!(result.is_ok() && result.unwrap() == None);
}

