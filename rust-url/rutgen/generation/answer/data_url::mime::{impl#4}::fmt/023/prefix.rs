// Answer 0

#[test]
fn test_fmt_with_invalid_value_in_parameters() {
    let mime = Mime {
        type_: String::from("text"),
        subtype: String::from("html"),
        parameters: vec![
            (String::from("charset"), String::from("utf-8")),
            (String::from("type"), String::from("invalid value with \"quotes\"")),
        ],
    };
    let mut buf = String::new();
    let _ = write!(&mut buf, "{}", mime);
}

