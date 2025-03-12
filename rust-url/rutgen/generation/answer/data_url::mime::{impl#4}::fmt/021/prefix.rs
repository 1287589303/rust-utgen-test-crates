// Answer 0

#[test]
fn test_fmt_with_invalid_value() {
    let mime = Mime {
        type_: String::from("text"),
        subtype: String::from("plain"),
        parameters: vec![(String::from("charset"), String::from("utf-8/invalid"))],
    };
    let mut output = String::new();
    let _ = write!(&mut output, "{}", mime);
}

#[test]
fn test_fmt_with_multiple_parameters_invalid_value() {
    let mime = Mime {
        type_: String::from("application"),
        subtype: String::from("json"),
        parameters: vec![
            (String::from("version"), String::from("1.0")),
            (String::from("token"), String::from("invalid/value")),
        ],
    };
    let mut output = String::new();
    let _ = write!(&mut output, "{}", mime);
}

