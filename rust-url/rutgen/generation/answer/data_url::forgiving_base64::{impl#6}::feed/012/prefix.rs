// Answer 0

#[test]
fn test_feed_with_only_spaces() {
    let mut output: Vec<u8> = Vec::new();
    let mut decoder = Decoder::new(|bytes| {
        output.extend_from_slice(bytes);
        Ok(())
    });
    decoder.feed(&[b' ', b' ']).unwrap();
}

#[test]
fn test_feed_with_newline() {
    let mut output: Vec<u8> = Vec::new();
    let mut decoder = Decoder::new(|bytes| {
        output.extend_from_slice(bytes);
        Ok(())
    });
    decoder.feed(&[b'\n']).unwrap();
}

#[test]
fn test_feed_with_carriage_return() {
    let mut output: Vec<u8> = Vec::new();
    let mut decoder = Decoder::new(|bytes| {
        output.extend_from_slice(bytes);
        Ok(())
    });
    decoder.feed(&[b'\r']).unwrap();
}

#[test]
fn test_feed_with_tab() {
    let mut output: Vec<u8> = Vec::new();
    let mut decoder = Decoder::new(|bytes| {
        output.extend_from_slice(bytes);
        Ok(())
    });
    decoder.feed(&[b'\t']).unwrap();
}

#[test]
fn test_feed_with_form_feed() {
    let mut output: Vec<u8> = Vec::new();
    let mut decoder = Decoder::new(|bytes| {
        output.extend_from_slice(bytes);
        Ok(())
    });
    decoder.feed(&[b'\x0C']).unwrap();
}

