// Answer 0

#[test]
fn test_decode_to_vec_valid_length_4n() {
    let input = b"QUJD"; // "ABC"
    let _ = decode_to_vec(input);
}

#[test]
fn test_decode_to_vec_valid_length_4n_plus_2() {
    let input = b"QUJD=="; // "ABC"
    let _ = decode_to_vec(input);
}

#[test]
fn test_decode_to_vec_valid_length_4n_plus_4() {
    let input = b"QUJD==AA"; // "ABC"
    let _ = decode_to_vec(input);
}

#[test]
fn test_decode_to_vec_valid_with_whitespace() {
    let input = b"Q \n U\tJ\tD"; // "ABC"
    let _ = decode_to_vec(input);
}

#[test]
fn test_decode_to_vec_empty_string() {
    let input = b""; // Empty input
    let _ = decode_to_vec(input);
}

#[test]
#[should_panic] // Expecting a panic due to invalid input causing finish to return Err
fn test_decode_to_vec_invalid_character() {
    let input = b"QUJD#"; // Invalid base64 character '#'
    let _ = decode_to_vec(input);
}

#[test]
#[should_panic] // Expecting a panic due to padding issues
fn test_decode_to_vec_lone_alphabet_symbol() {
    let input = b"Q"; // Lone symbol without necessary padding
    let _ = decode_to_vec(input);
}

