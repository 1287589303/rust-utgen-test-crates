// Answer 0

#[test]
fn test_fmt_with_valid_type_and_subtype() {
    let mime = Mime {
        type_: String::from("text"),
        subtype: String::from("plain"),
        parameters: vec![(String::from("charset"), String::from(""))],
    };
    let mut output = String::new();
    let _ = write!(&mut output, "{}", mime);
}

#[test]
fn test_fmt_with_valid_type_and_subtype_empty_value() {
    let mime = Mime {
        type_: String::from("application"),
        subtype: String::from("json"),
        parameters: vec![(String::from("foo"), String::from(""))],
    };
    let mut output = String::new();
    let _ = write!(&mut output, "{}", mime);
}

#[test]
fn test_fmt_with_invalid_value() {
    let mime = Mime {
        type_: String::from("image"),
        subtype: String::from("png"),
        parameters: vec![(String::from("size"), String::from("invalid value"))],
    };
    let mut output = String::new();
    let _ = write!(&mut output, "{}", mime);
}

