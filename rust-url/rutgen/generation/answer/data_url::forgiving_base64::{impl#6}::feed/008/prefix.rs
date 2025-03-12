// Answer 0

#[test]
fn test_feed_with_valid_base64_then_invalid_character() {
    let mut called = false;
    let write_bytes = |bytes: &[u8]| {
        called = true;
        Ok(())
    };
    let mut decoder = Decoder::new(write_bytes);
    let input: &[u8] = b"QUJD=!";
    let result = decoder.feed(input);
}

#[test]
fn test_feed_with_valid_base64_followed_by_padding() {
    let mut called = false;
    let write_bytes = |bytes: &[u8]| {
        called = true;
        Ok(())
    };
    let mut decoder = Decoder::new(write_bytes);
    let input: &[u8] = b"QUJD==";
    let result = decoder.feed(input);
}

