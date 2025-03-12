// Answer 0

#[test]
fn test_into_iter_empty_token_stream() {
    let token_stream = TokenStream::new();
    let _iter = token_stream.into_iter();
}

#[test]
fn test_into_iter_non_empty_token_stream() {
    let token_stream = TokenStream::from_str_unchecked("let x = 5;");
    let _iter = token_stream.into_iter();
}

#[test]
fn test_into_iter_empty_string() {
    let token_stream = TokenStream::from_str_checked("").unwrap();
    let _iter = token_stream.into_iter();
}

#[test]
fn test_into_iter_byte_order_mark_string() {
    let token_stream = TokenStream::from_str_checked("\u{feff}").unwrap();
    let _iter = token_stream.into_iter();
}

#[test]
fn test_into_iter_identifier_only() {
    let token_stream = TokenStream::from_str_checked("identifier").unwrap();
    let _iter = token_stream.into_iter();
}

#[test]
fn test_into_iter_literal_only() {
    let token_stream = TokenStream::from_str_checked("42").unwrap();
    let _iter = token_stream.into_iter();
}

#[test]
fn test_into_iter_punctuation_only() {
    let token_stream = TokenStream::from_str_checked("+").unwrap();
    let _iter = token_stream.into_iter();
}

