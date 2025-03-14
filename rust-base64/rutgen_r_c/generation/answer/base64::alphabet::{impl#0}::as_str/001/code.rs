// Answer 0

#[test]
fn test_standard_as_str() {
    let alphabet = crate::STANDARD;
    let result = alphabet.as_str();
    assert_eq!(result, "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
}

#[test]
fn test_url_safe_as_str() {
    let alphabet = crate::URL_SAFE;
    let result = alphabet.as_str();
    assert_eq!(result, "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_");
}

#[test]
fn test_crypt_as_str() {
    let alphabet = crate::CRYPT;
    let result = alphabet.as_str();
    assert_eq!(result, "./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz");
}

#[test]
fn test_bcrypt_as_str() {
    let alphabet = crate::BCRYPT;
    let result = alphabet.as_str();
    assert_eq!(result, "./ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");
}

#[test]
fn test_imap_mut7_as_str() {
    let alphabet = crate::IMAP_MUTF7;
    let result = alphabet.as_str();
    assert_eq!(result, "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+,,");
}

#[test]
fn test_bin_hex_as_str() {
    let alphabet = crate::BIN_HEX;
    let result = alphabet.as_str();
    assert_eq!(result, "!\"#$%&'()*+,-012345689@ABCDEFGHIJKLMNPQRSTUVXYZ[`abcdefhijklmpqr");
}

