// Answer 0

#[test]
fn test_from_str_unchecked_valid() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let _result = Alphabet::from_str_unchecked(alphabet);
}

#[test]
fn test_from_str_unchecked_valid_with_url_safe() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let _result = Alphabet::from_str_unchecked(alphabet);
}

#[test]
fn test_from_str_unchecked_valid_with_crypt() {
    let alphabet = "./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let _result = Alphabet::from_str_unchecked(alphabet);
}

#[test]
fn test_from_str_unchecked_valid_with_bcrypt() {
    let alphabet = "./ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let _result = Alphabet::from_str_unchecked(alphabet);
}

#[test]
fn test_from_str_unchecked_valid_with_imap_mut7() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+,";
    let _result = Alphabet::from_str_unchecked(alphabet);
}

#[test]
fn test_from_str_unchecked_valid_with_bin_hex() {
    let alphabet = "!\"#$%&'()*+,-012345689@ABCDEFGHIJKLMNPQRSTUVXYZ[`abcdefhijklmpqr";
    let _result = Alphabet::from_str_unchecked(alphabet);
}

#[test]
#[should_panic]
fn test_from_str_unchecked_invalid_too_short() {
    let alphabet = "short";
    let _result = Alphabet::from_str_unchecked(alphabet);
}

#[test]
#[should_panic]
fn test_from_str_unchecked_invalid_too_long() {
    let alphabet = "this string is definitely longer than sixty-four characters long.";
    let _result = Alphabet::from_str_unchecked(alphabet);
}

#[test]
#[should_panic]
fn test_from_str_unchecked_invalid_empty() {
    let alphabet = "";
    let _result = Alphabet::from_str_unchecked(alphabet);
}

