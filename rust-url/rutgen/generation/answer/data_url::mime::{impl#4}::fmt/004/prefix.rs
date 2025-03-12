// Answer 0

#[test]
fn test_fmt_valid_type_and_subtype_with_err_on_semi_colon() {
    let mime = Mime {
        type_: String::from("text"),
        subtype: String::from("plain"),
        parameters: vec![
            (String::from("charset"), String::from("utf-8")),
            (String::new(), String::from("value")), // This should trigger an error on f.write_str(";")
        ],
    };
    let _ = format!("{}", mime);
}

#[test]
fn test_fmt_with_special_characters_in_parameters() {
    let mime = Mime {
        type_: String::from("application"),
        subtype: String::from("json"),
        parameters: vec![
            (String::from("type"), String::from("data")),
            (String::from("example"), String::from("value with \"quote\" and \\ backslash")),
        ],
    };
    let _ = format!("{}", mime);
}

#[test]
fn test_fmt_with_http_token_value() {
    let mime = Mime {
        type_: String::from("image"),
        subtype: String::from("png"),
        parameters: vec![
            (String::from("name"), String::from("value")),
        ],
    };
    let _ = format!("{}", mime);
}

#[test]
fn test_fmt_with_empty_value() {
    let mime = Mime {
        type_: String::from("audio"),
        subtype: String::from("wav"),
        parameters: vec![
            (String::from("type"), String::from("")),
        ],
    };
    let _ = format!("{}", mime);
}

