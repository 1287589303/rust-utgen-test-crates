// Answer 0

#[test]
fn test_decoded_len_estimate_1() {
    assert_eq!(3, base64::decoded_len_estimate(1));
}

#[test]
fn test_decoded_len_estimate_2() {
    assert_eq!(3, base64::decoded_len_estimate(2));
}

#[test]
fn test_decoded_len_estimate_3() {
    assert_eq!(3, base64::decoded_len_estimate(3));
}

#[test]
fn test_decoded_len_estimate_4() {
    assert_eq!(3, base64::decoded_len_estimate(4));
}

#[test]
fn test_decoded_len_estimate_5() {
    assert_eq!(6, base64::decoded_len_estimate(5));
}

