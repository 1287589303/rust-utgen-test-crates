// Answer 0

#[test]
fn test_not_a_data_url() {
    let error = DataUrlError::NotADataUrl;
    let mut output = String::new();
    let fmt_result = error.fmt(&mut output);
}

#[test]
fn test_no_comma() {
    let error = DataUrlError::NoComma;
    let mut output = String::new();
    let fmt_result = error.fmt(&mut output);
}

