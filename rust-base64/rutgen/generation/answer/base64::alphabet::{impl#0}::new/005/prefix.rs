// Answer 0

#[test]
fn test_valid_alphabet_creation() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let result = Alphabet::new(alphabet);
}

#[test]
fn test_alphabet_creation_with_min_bound() {
    let alphabet = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[^_`abcdefghijklmnopqrstuvwxyz{|}~";
    let result = Alphabet::new(alphabet);
}

#[test]
fn test_alphabet_creation_with_max_bound() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-~";
    let result = Alphabet::new(alphabet);
}

#[test]
fn test_alphabet_creation_without_pad_byte() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let result = Alphabet::new(alphabet);
}

