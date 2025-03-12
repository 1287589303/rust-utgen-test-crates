// Answer 0

#[test]
fn test_feed_with_unexpected_symbol() {
    let mut seen_bytes: Vec<u8> = Vec::new();
    let write_fn = |bytes: &[u8]| {
        seen_bytes.extend_from_slice(bytes);
        Ok(())
    };
    
    let mut decoder = Decoder::new(write_fn);
    
    // Testing various unexpected symbols
    let input = [0x00, 0x01, 0xFF];
    let _ = decoder.feed(&input).unwrap_err();
}

#[test]
fn test_feed_with_whitespace() {
    let mut seen_bytes: Vec<u8> = Vec::new();
    let write_fn = |bytes: &[u8]| {
        seen_bytes.extend_from_slice(bytes);
        Ok(())
    };

    let mut decoder = Decoder::new(write_fn);
    
    // Testing whitespace characters
    let input = [b'\n', b'\r', b'\x0C', b'\t', b' '];
    let _ = decoder.feed(&input).unwrap_err();
}

#[test]
fn test_feed_with_padding() {
    let mut seen_bytes: Vec<u8> = Vec::new();
    let write_fn = |bytes: &[u8]| {
        seen_bytes.extend_from_slice(bytes);
        Ok(())
    };

    let mut decoder = Decoder::new(write_fn);
    
    // Testing padding character
    let input = [b'='];
    let _ = decoder.feed(&input).unwrap_err();
}

