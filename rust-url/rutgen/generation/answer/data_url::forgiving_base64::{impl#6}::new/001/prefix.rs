// Answer 0

#[test]
fn test_decoder_new_with_valid_callback() {
    let callback = |_: &[u8]| -> Result<(), ()> { Ok(()) };
    let decoder: Decoder<_, ()> = Decoder::new(callback);
}

#[test]
fn test_decoder_new_with_empty_callback() {
    let callback = |input: &[u8]| -> Result<(), ()> {
        assert!(input.is_empty());
        Ok(())
    };
    let decoder: Decoder<_, ()> = Decoder::new(callback);
}

#[test]
fn test_decoder_new_with_invalid_callback() {
    let callback = |_: &[u8]| -> Result<(), &'static str> { Err("error") };
    let decoder: Decoder<_, _> = Decoder::new(callback);
}

#[test]
fn test_decoder_new_with_large_callback() {
    let callback = |data: &[u8]| -> Result<(), ()> {
        assert!(data.len() > 100);
        Ok(())
    };
    let data = vec![0u8; 150];
    let decoder: Decoder<_, ()> = Decoder::new(callback);
    let _ = decoder.feed(&data);
}

