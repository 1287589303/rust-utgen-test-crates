// Answer 0

#[test]
fn test_decode_empty_slice() {
    let input: &[u8] = &[];
    decode(input);
}

#[test]
fn test_decode_empty_vec() {
    let input: Vec<u8> = Vec::new();
    decode(input);
}

