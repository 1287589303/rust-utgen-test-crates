// Answer 0

#[test]
fn test_decoded_len_estimate_for_one_encoded_symbol() {
    assert_eq!(3, decoded_len_estimate(1));
}

#[test]
fn test_decoded_len_estimate_for_two_encoded_symbols() {
    assert_eq!(3, decoded_len_estimate(2));
}

#[test]
fn test_decoded_len_estimate_for_three_encoded_symbols() {
    assert_eq!(3, decoded_len_estimate(3));
}

#[test]
fn test_decoded_len_estimate_for_four_encoded_symbols() {
    assert_eq!(3, decoded_len_estimate(4));
}

#[test]
fn test_decoded_len_estimate_for_five_encoded_symbols() {
    assert_eq!(6, decoded_len_estimate(5));
}

