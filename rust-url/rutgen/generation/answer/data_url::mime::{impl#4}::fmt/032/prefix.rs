// Answer 0

#[test]
fn test_mime_display_with_double_quote_in_value() {
    let mime = Mime {
        type_: "text".to_owned(),
        subtype: "plain".to_owned(),
        parameters: vec![("charset".to_owned(), "\"test\"".to_owned())],
    };
    let _ = format!("{}", mime);
}

#[test]
fn test_mime_display_with_backslash_in_value() {
    let mime = Mime {
        type_: "image".to_owned(),
        subtype: "png".to_owned(),
        parameters: vec![("file".to_owned(), "\\image\\path".to_owned())],
    };
    let _ = format!("{}", mime);
}

#[test]
fn test_mime_display_with_double_quote_and_backslash_in_value() {
    let mime = Mime {
        type_: "application".to_owned(),
        subtype: "json".to_owned(),
        parameters: vec![("data".to_owned(), "example\"text\\data".to_owned())],
    };
    let _ = format!("{}", mime);
}

