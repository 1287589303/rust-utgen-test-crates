// Answer 0

#[test]
fn test_syntax_error_valid_pattern() {
    let err = regex_syntax::Error::new("valid_pattern".to_string());
    let build_error = BuildError::syntax(err);
}

#[test]
fn test_syntax_error_invalid_pattern() {
    let err = regex_syntax::Error::new("invalid_pattern$".to_string());
    let build_error = BuildError::syntax(err);
}

#[test]
fn test_syntax_error_empty_pattern() {
    let err = regex_syntax::Error::new("".to_string());
    let build_error = BuildError::syntax(err);
}

#[test]
fn test_syntax_error_pattern_with_special_symbols() {
    let err = regex_syntax::Error::new("pattern_with_!@#%^&*()".to_string());
    let build_error = BuildError::syntax(err);
}

#[test]
fn test_syntax_error_complex_pattern() {
    let err = regex_syntax::Error::new("complex_pattern[abc]{2,3}".to_string());
    let build_error = BuildError::syntax(err);
}

