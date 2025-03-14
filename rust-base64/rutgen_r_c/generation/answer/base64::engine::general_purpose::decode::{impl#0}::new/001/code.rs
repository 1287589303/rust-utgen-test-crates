// Answer 0

#[test]
fn test_new_with_zero_encoded_len() {
    let estimate = GeneralPurposeEstimate::new(0);
    assert_eq!(estimate.rem, 0);
    assert_eq!(estimate.conservative_decoded_len, 0);
}

#[test]
fn test_new_with_non_four_multiple_encoded_len() {
    let estimate = GeneralPurposeEstimate::new(5);
    assert_eq!(estimate.rem, 1);
    assert_eq!(estimate.conservative_decoded_len, 6);
}

#[test]
fn test_new_with_four_multiple_encoded_len() {
    let estimate = GeneralPurposeEstimate::new(8);
    assert_eq!(estimate.rem, 0);
    assert_eq!(estimate.conservative_decoded_len, 6);
}

#[test]
fn test_new_with_one_encoded_len() {
    let estimate = GeneralPurposeEstimate::new(1);
    assert_eq!(estimate.rem, 1);
    assert_eq!(estimate.conservative_decoded_len, 3);
}

#[test]
fn test_new_with_negative_boundary_encoded_len() {
    let estimate = GeneralPurposeEstimate::new(3);
    assert_eq!(estimate.rem, 3);
    assert_eq!(estimate.conservative_decoded_len, 3);
}

