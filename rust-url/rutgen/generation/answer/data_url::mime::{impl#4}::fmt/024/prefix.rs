// Answer 0

#[test]
fn test_fmt_with_invalid_value_characters() {
    let mime = Mime {
        type_: "text".to_owned(),
        subtype: "plain".to_owned(),
        parameters: vec![("charset".to_owned(), "utf-8\"extra".to_owned())],
    };
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", mime);
}

#[test]
fn test_fmt_with_multiple_invalid_value_characters() {
    let mime = Mime {
        type_: "image".to_owned(),
        subtype: "png".to_owned(),
        parameters: vec![
            ("filename".to_owned(), "image\\file\"name".to_owned()),
            ("width".to_owned(), "500\"px\\value".to_owned()),
        ],
    };
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", mime);
}

#[test]
fn test_fmt_with_unicode_in_value() {
    let mime = Mime {
        type_: "application".to_owned(),
        subtype: "json".to_owned(),
        parameters: vec![("data".to_owned(), "value_with_chars_©®".to_owned())],
    };
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", mime);
}

#[test]
fn test_fmt_with_special_characters() {
    let mime = Mime {
        type_: "text".to_owned(),
        subtype: "csv".to_owned(),
        parameters: vec![("quote".to_owned(), "text\"and\\characters".to_owned())],
    };
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", mime);
}

#[test]
fn test_fmt_with_leading_trailing_spaces_in_value() {
    let mime = Mime {
        type_: "text".to_owned(),
        subtype: "html".to_owned(),
        parameters: vec![("value".to_owned(), " leading \\trailing\" ".to_owned())],
    };
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", mime);
}

