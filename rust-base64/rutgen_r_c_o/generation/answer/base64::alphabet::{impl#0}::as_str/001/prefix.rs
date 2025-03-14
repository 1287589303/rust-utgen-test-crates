// Answer 0

#[test]
fn test_as_str_standard_alphabet() {
    let alphabet = Alphabet::from_str_unchecked("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
    let result = alphabet.as_str();
}

#[test]
fn test_as_str_url_safe_alphabet() {
    let alphabet = Alphabet::from_str_unchecked("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_");
    let result = alphabet.as_str();
}

#[test]
fn test_as_str_crypt_alphabet() {
    let alphabet = Alphabet::from_str_unchecked("./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz");
    let result = alphabet.as_str();
}

#[test]
fn test_as_str_bcrypt_alphabet() {
    let alphabet = Alphabet::from_str_unchecked("./ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");
    let result = alphabet.as_str();
}

#[test]
fn test_as_str_imap_mut7_alphabet() {
    let alphabet = Alphabet::from_str_unchecked("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+,");
    let result = alphabet.as_str();
}

#[test]
fn test_as_str_bin_hex_alphabet() {
    let alphabet = Alphabet::from_str_unchecked("!\"#$%&'()*+,-012345689@ABCDEFGHIJKLMNPQRSTUVXYZ[`abcdefhijklmpqr");
    let result = alphabet.as_str();
}

#[test]
fn test_as_str_custom_valid_alphabet() {
    let alphabet = Alphabet::new("1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz").unwrap();
    let result = alphabet.as_str();
}

