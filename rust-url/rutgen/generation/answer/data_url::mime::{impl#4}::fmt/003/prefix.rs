// Answer 0

#[test]
fn test_fmt_valid_type_empty_subtype() {
    let mime = Mime {
        type_: "text".to_owned(),
        subtype: "".to_owned(),
        parameters: vec![],
    };
    let mut output = String::new();
    let _ = write!(&mut output, "{}", mime);
}

#[test]
fn test_fmt_valid_type_valid_subtype() {
    let mime = Mime {
        type_: "text".to_owned(),
        subtype: "".to_owned(),
        parameters: vec![("charset".to_owned(), "utf-8".to_owned())],
    };
    let mut output = String::new();
    let _ = write!(&mut output, "{}", mime);
}

#[test]
fn test_fmt_valid_type_valid_subtype_invalid_value() {
    let mime = Mime {
        type_: "text".to_owned(),
        subtype: "".to_owned(),
        parameters: vec![("filename".to_owned(), "inv@lid".to_owned())],
    };
    let mut output = String::new();
    let _ = write!(&mut output, "{}", mime);
}

#[test]
fn test_fmt_valid_type_empty_subtype_non_empty_parameters() {
    let mime = Mime {
        type_: "image".to_owned(),
        subtype: "".to_owned(),
        parameters: vec![
            ("size".to_owned(), "small".to_owned()),
            ("quality".to_owned(), "high".to_owned()),
        ],
    };
    let mut output = String::new();
    let _ = write!(&mut output, "{}", mime);
}

#[test]
fn test_fmt_valid_type_empty_subtype_escape_value() {
    let mime = Mime {
        type_: "application".to_owned(),
        subtype: "".to_owned(),
        parameters: vec![("name".to_owned(), "my\"file".to_owned())],
    };
    let mut output = String::new();
    let _ = write!(&mut output, "{}", mime);
}

