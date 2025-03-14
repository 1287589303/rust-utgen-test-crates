// Answer 0

#[test]
fn test_new_with_zero_length() {
    let estimate = GeneralPurposeEstimate::new(0);
    assert_eq!(estimate.rem, 0);
    assert_eq!(estimate.conservative_decoded_len, 0);
}

#[test]
fn test_new_with_length_that_is_multiple_of_four() {
    let estimate = GeneralPurposeEstimate::new(8);
    assert_eq!(estimate.rem, 0);
    assert_eq!(estimate.conservative_decoded_len, 6);
}

#[test]
fn test_new_with_length_one_more_than_multiple_of_four() {
    let estimate = GeneralPurposeEstimate::new(5);
    assert_eq!(estimate.rem, 1);
    assert_eq!(estimate.conservative_decoded_len, 6);
}

#[test]
fn test_new_with_length_two_more_than_multiple_of_four() {
    let estimate = GeneralPurposeEstimate::new(6);
    assert_eq!(estimate.rem, 2);
    assert_eq!(estimate.conservative_decoded_len, 6);
}

#[test]
fn test_new_with_length_three_more_than_multiple_of_four() {
    let estimate = GeneralPurposeEstimate::new(7);
    assert_eq!(estimate.rem, 3);
    assert_eq!(estimate.conservative_decoded_len, 9);
}

#[test]
fn test_new_with_large_length() {
    let estimate = GeneralPurposeEstimate::new(100);
    assert_eq!(estimate.rem, 0);
    assert_eq!(estimate.conservative_decoded_len, 75);
}

