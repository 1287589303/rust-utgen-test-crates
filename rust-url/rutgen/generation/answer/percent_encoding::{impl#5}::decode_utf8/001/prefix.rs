// Answer 0

#[test]
fn test_decode_utf8_invalid_sequence() {
    let bytes: Vec<u8> = vec![0xFF, 0xFE, 0xFD];
    let decoder = PercentDecode { bytes: bytes.iter() };
    let result = decoder.decode_utf8();
}

#[test]
fn test_decode_utf8_another_invalid_sequence() {
    let bytes: Vec<u8> = vec![0x80, 0x81, 0xFE];
    let decoder = PercentDecode { bytes: bytes.iter() };
    let result = decoder.decode_utf8();
}

