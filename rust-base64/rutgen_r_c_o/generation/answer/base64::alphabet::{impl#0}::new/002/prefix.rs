// Answer 0

#[test]
fn test_new_alphabet_with_reserved_byte() {
    const PAD_BYTE: u8 = b'='; // Assuming '=' is the padding byte
    let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let input_with_reserved = format!("{}{}", input, PAD_BYTE as char);
    let result = Alphabet::new(&input_with_reserved);
}

#[test]
fn test_new_alphabet_with_bound_low_byte() {
    let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let input_with_bound_low = format!("{}{}", b'\x20' as char, input);
    let result = Alphabet::new(&input_with_bound_low);
}

#[test]
fn test_new_alphabet_with_bound_high_byte() {
    let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let input_with_bound_high = format!("{}{}", b'\x7E' as char, input);
    let result = Alphabet::new(&input_with_bound_high);
}

#[test]
fn test_new_alphabet_exceeding_size() {
    let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
    let result = Alphabet::new(input);
}

