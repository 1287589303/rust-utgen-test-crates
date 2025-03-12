// Answer 0

#[test]
fn test_decode_without_base64_no_special_bytes() {
    let input = "Hello World%20#fragment";
    let mut output = Vec::new();
    decode_without_base64(input, |bytes| {
        output.extend_from_slice(bytes);
        Ok(())
    }).unwrap();
}

#[test]
fn test_decode_without_base64_partial_percent_encoding() {
    let input = "Hello%20World%#fragment";
    let mut output = Vec::new();
    decode_without_base64(input, |bytes| {
        output.extend_from_slice(bytes);
        Ok(())
    }).unwrap();
}

#[test]
fn test_decode_without_base64_only_fragment() {
    let input = "Hello World#fragment";
    let mut output = Vec::new();
    decode_without_base64(input, |bytes| {
        output.extend_from_slice(bytes);
        Ok(())
    }).unwrap();
}

#[test]
fn test_decode_without_base64_with_newline() {
    let input = "Hello World\n#fragment";
    let mut output = Vec::new();
    decode_without_base64(input, |bytes| {
        output.extend_from_slice(bytes);
        Ok(())
    }).unwrap();
}

#[test]
fn test_decode_without_base64_with_tab() {
    let input = "Hello World\t#fragment";
    let mut output = Vec::new();
    decode_without_base64(input, |bytes| {
        output.extend_from_slice(bytes);
        Ok(())
    }).unwrap();
}

#[test]
fn test_decode_without_base64_multiple_specials() {
    let input = "Hello%20World%#frag#ment";
    let mut output = Vec::new();
    decode_without_base64(input, |bytes| {
        output.extend_from_slice(bytes);
        Ok(())
    }).unwrap();
}

#[test]
fn test_decode_without_base64_only_percent_and_fragment() {
    let input = "%20#fragment";
    let mut output = Vec::new();
    decode_without_base64(input, |bytes| {
        output.extend_from_slice(bytes);
        Ok(())
    }).unwrap();
}

#[test]
fn test_decode_without_base64_only_percent() {
    let input = "%20";
    let mut output = Vec::new();
    decode_without_base64(input, |bytes| {
        output.extend_from_slice(bytes);
        Ok(())
    }).unwrap();
}

