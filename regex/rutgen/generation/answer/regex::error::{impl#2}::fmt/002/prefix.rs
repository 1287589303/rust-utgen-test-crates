// Answer 0

#[test]
fn test_error_syntax_empty_string() {
    let error = Error::Syntax(String::from(""));
    let mut f = core::fmt::Formatter::new();
    let _ = error.fmt(&mut f);
}

#[test]
fn test_error_syntax_typical_length() {
    let error = Error::Syntax(String::from("A typical error message."));
    let mut f = core::fmt::Formatter::new();
    let _ = error.fmt(&mut f);
}

#[test]
fn test_error_syntax_maximum_length() {
    let error = Error::Syntax(String::from("A".repeat(255)));
    let mut f = core::fmt::Formatter::new();
    let _ = error.fmt(&mut f);
}

#[test]
fn test_error_syntax_over_maximum_length() {
    let error = Error::Syntax(String::from("A".repeat(260)));
    let mut f = core::fmt::Formatter::new();
    let _ = error.fmt(&mut f);
}

