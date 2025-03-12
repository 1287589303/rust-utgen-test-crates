// Answer 0

#[test]
fn test_from_str_unchecked_boundary_condition() {
    let input_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let alphabet = Alphabet::from_str_unchecked(input_alphabet);
}

#[test]
fn test_from_str_unchecked_boundary_condition_with_url_safe() {
    let input_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let alphabet = Alphabet::from_str_unchecked(input_alphabet);
}

#[test]
fn test_from_str_unchecked_boundary_condition_with_crypt() {
    let input_alphabet = "./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let alphabet = Alphabet::from_str_unchecked(input_alphabet);
}

#[test]
fn test_from_str_unchecked_boundary_condition_with_bcrypt() {
    let input_alphabet = "./ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let alphabet = Alphabet::from_str_unchecked(input_alphabet);
}

#[test]
fn test_from_str_unchecked_boundary_condition_with_imap_mut7() {
    let input_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+,";
    let alphabet = Alphabet::from_str_unchecked(input_alphabet);
}

#[test]
fn test_from_str_unchecked_boundary_condition_with_bin_hex() {
    let input_alphabet = "!\"#$%&'()*+,-012345689@ABCDEFGHIJKLMNPQRSTUVXYZ[`abcdefhijklmpqr";
    let alphabet = Alphabet::from_str_unchecked(input_alphabet);
}

