// Answer 0

#[test]
fn test_decoded_len_estimate_single_byte() {
    assert_eq!(3, decoded_len_estimate(1));
}

#[test]
fn test_decoded_len_estimate_two_bytes() {
    assert_eq!(3, decoded_len_estimate(2));
}

#[test]
fn test_decoded_len_estimate_three_bytes() {
    assert_eq!(3, decoded_len_estimate(3));
}

#[test]
fn test_decoded_len_estimate_four_bytes() {
    assert_eq!(3, decoded_len_estimate(4));
}

#[test]
fn test_decoded_len_estimate_five_bytes() {
    assert_eq!(6, decoded_len_estimate(5));
}

