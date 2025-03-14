// Answer 0

#[test]
fn test_valid_alphabet_creation() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let result = Alphabet::new(alphabet);
}

#[test]
fn test_valid_alphabet_creation_url_safe() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let result = Alphabet::new(alphabet);
}

#[test]
fn test_valid_alphabet_creation_crypt() {
    let alphabet = "./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let result = Alphabet::new(alphabet);
}

#[test]
fn test_valid_alphabet_creation_bcrypt() {
    let alphabet = "./ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let result = Alphabet::new(alphabet);
}

#[test]
fn test_valid_alphabet_creation_imap_mut7() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+,";
    let result = Alphabet::new(alphabet);
}

#[test]
fn test_valid_alphabet_creation_bin_hex() {
    let alphabet = "!\"#$%&'()*+,-012345689@ABCDEFGHIJKLMNPQRSTUVXYZ[`abcdefhijklmpqr";
    let result = Alphabet::new(alphabet);
}

