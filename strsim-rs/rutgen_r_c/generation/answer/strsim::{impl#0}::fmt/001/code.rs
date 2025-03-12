// Answer 0

#[test]
fn test_display_different_length_args() {
    let error = StrSimError::DifferentLengthArgs;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);

    assert!(result.is_ok());
    assert_eq!(output, "Differing length arguments provided");
}

#[test]
fn test_display_error_format() {
    let error = StrSimError::DifferentLengthArgs;
    let expected_output = "Differing length arguments provided";

    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error);

    assert!(result.is_ok());
    assert_eq!(buffer, expected_output);
}

