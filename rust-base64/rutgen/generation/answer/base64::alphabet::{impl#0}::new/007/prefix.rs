// Answer 0

#[test]
fn test_new_invalid_unprintable_byte_above_boundary() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let alphabet = alphabet.replace('A', "\x7F"); // Replace 'A' (65) with unprintable byte (127)
    let result = Alphabet::new(&alphabet);
}

#[test]
fn test_new_invalid_unprintable_byte_at_boundary() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let alphabet = alphabet.replace('A', "\x7F"); // Replace 'A' with unprintable byte (127)
    let result = Alphabet::new(&alphabet);
}

