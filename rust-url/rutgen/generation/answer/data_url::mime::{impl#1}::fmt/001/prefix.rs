// Answer 0

#[test]
fn test_mime_parsing_error_fmt() {
    let error = MimeParsingError(());
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    let expected_output = "invalid mime type";
    result.unwrap();
    assert_eq!(output, expected_output);
}

#[test]
fn test_mime_parsing_error_fmt_boundary() {
    let error = MimeParsingError(());
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    let expected_output = "invalid mime type";
    result.unwrap();
    assert_eq!(output, expected_output);
}

