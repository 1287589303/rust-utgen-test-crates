// Answer 0

#[test]
fn test_display_mime_with_valid_http_token() {
    let mime = Mime {
        type_: String::from("text"),
        subtype: String::from("plain"),
        parameters: vec![(String::from("charset"), String::from("utf-8"))],
    };
    let _ = format!("{}", mime);
}

#[test]
fn test_display_mime_with_multiple_parameters() {
    let mime = Mime {
        type_: String::from("application"),
        subtype: String::from("json"),
        parameters: vec![
            (String::from("charset"), String::from("utf-8")),
            (String::from("version"), String::from("1.0")),
        ],
    };
    let _ = format!("{}", mime);
}

#[test]
fn test_display_mime_with_http_token_value_only() {
    let mime = Mime {
        type_: String::from("image"),
        subtype: String::from("png"),
        parameters: vec![(String::from("quality"), String::from("high"))],
    };
    let _ = format!("{}", mime);
}

#[test]
fn test_display_mime_with_empty_value() {
    let mime = Mime {
        type_: String::from("text"),
        subtype: String::from("html"),
        parameters: vec![(String::from("content-type"), String::from(""))],
    };
    let _ = format!("{}", mime);
}

